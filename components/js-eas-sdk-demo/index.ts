import { TriggerAction, WasmResponse } from "./out/wavs:worker@0.4.0-beta.4";
import { decodeTriggerEvent, encodeOutput, Destination } from "./trigger";
import { AbiCoder } from "ethers";
import { EASGraphQLClient } from "./eas-client";
import { testPolygonContainment } from "./spatial-analysis";

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

  const containmentResults = testPolygonContainment(attestations, locationAttestation);

  return new TextEncoder().encode(JSON.stringify(containmentResults));
}

export { run };
