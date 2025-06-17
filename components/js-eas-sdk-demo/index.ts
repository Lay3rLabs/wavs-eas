import { TriggerAction, WasmResponse } from "./out/wavs:worker@0.4.0-beta.4";
import { decodeTriggerEvent, encodeOutput, Destination } from "./trigger";
import { AbiCoder } from "ethers";
import booleanContains from "@turf/boolean-contains";
import { point, polygon } from "@turf/helpers";
import { getLocation } from "./data-utils";
import { EASGraphQLClient } from "./eas-client";

async function run(triggerAction: TriggerAction): Promise<WasmResponse> {
  let event = decodeTriggerEvent(triggerAction.data);
  let triggerId = event[0].triggerId;

  let result = await compute(event[0].data);

  switch (event[1]) {
    case Destination.Cli:
      return {
        payload: result,
        ordering: undefined,
      } as WasmResponse; // return raw bytes back
    case Destination.Ethereum:
      return {
        payload: encodeOutput(triggerId, result),
        ordering: undefined,
      } as WasmResponse; // return encoded bytes back
    case Destination.Cosmos:
      break;
  }

  throw new Error(
    "Unknown destination: " + event[1] + " for trigger ID: " + triggerId
  );
}

async function compute(input: Uint8Array): Promise<Uint8Array> {
  const abiCoder = new AbiCoder();
  const [chainId, attestationId] = abiCoder.decode(["uint256", "string"], input);

  const client = new EASGraphQLClient();
  const { attestations, locationAttestation } = await client.fetchAttestations(Number(chainId), attestationId);
  console.log(`\nreceived ${attestations.length} attestations\n`);

  if (!locationAttestation) {
    throw new Error("No location attestation found for polygon containment testing");
  }

  // Extract polygon from location attestation
  const locationAttestationJson = JSON.parse(locationAttestation.data);
  const polygonData = getLocation(locationAttestationJson);
  console.log(`\npolygon data from location attestation\n`, JSON.stringify(polygonData, null, 2));

  // Create turf polygon from the location attestation
  const turfPolygon = polygon(polygonData.coordinates);

  const containmentResults = attestations.map((attestation, index) => {
    const attestationJson = JSON.parse(attestation.data);
    console.log(`\ndecoded attestation ${index + 1} data\n`, JSON.stringify(attestationJson, null, 2));

    const locationData = getLocation(attestationJson);
    console.log(`\nextracted location from attestation ${index + 1}\n`, JSON.stringify(locationData, null, 2));

    // Create turf point from the attestation location
    const turfPoint = point(locationData.coordinates);

    // Test containment
    const isContained = booleanContains(turfPolygon, turfPoint);
    console.log(`\nattestation ${index + 1} point ${isContained ? 'IS' : 'IS NOT'} contained in polygon\n`);

    return {
      attestationId: attestation.uid,
      location: locationData,
      isContainedInPolygon: isContained
    };
  });

  console.log(`\ncontainment test results (${containmentResults.length} total)\n`, JSON.stringify(containmentResults, null, 2));

  return new TextEncoder().encode(JSON.stringify(containmentResults));
}

export { run };
