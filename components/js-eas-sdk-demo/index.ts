import { TriggerAction, WasmResponse } from "./out/wavs:worker@0.4.0-beta.4";
import { decodeTriggerEvent, encodeOutput, Destination } from "./trigger";
import { ethers } from "ethers";
import { AbiCoder } from "ethers";
import { hexlify } from "ethers";

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

interface AttestationInput {
  chainId: number;
  attestationId: string;
}

async function compute(input: Uint8Array): Promise<Uint8Array> {
  const abiCoder = new AbiCoder();
  const [chainId, attestationId] = abiCoder.decode(["uint256", "string"], input);

  const attestation = await fetchAttestations(Number(chainId), attestationId);
  console.log("\nreceived attestation\n", JSON.stringify(attestation, null, 2));

  const attestationJson = JSON.parse(attestation.data);
  console.log("\ndecoded attestation data\n", JSON.stringify(attestationJson, null, 2));

  const locationData = extractLocationFromAttestation(attestationJson);
  console.log("\nextracted location data\n", JSON.stringify(locationData, null, 2));

  return new TextEncoder().encode(JSON.stringify(locationData));
}

// ======================== EAS GraphQL ========================

interface AttestationData {
  uid: string;
  schemaId: string;
  refUID: string;
  time: number;
  expirationTime: number;
  revocationTime: number;
  recipient: string;
  attester: string;
  revocable: boolean;
  data: string;
  chainId: number;
}

/**
 * Converts a raw attestation object from GraphQL response to AttestationData format
 * @param rawAttestation The raw attestation object from GraphQL
 * @param chainId The chain ID to include in the result
 * @returns AttestationData object
 */
function convertRawAttestationToData(rawAttestation: any, chainId: number): AttestationData {
  return {
    uid: rawAttestation.id,
    schemaId: rawAttestation.schemaId,
    refUID: rawAttestation.refUID,
    time: Number(rawAttestation.time),
    expirationTime: Number(rawAttestation.expirationTime),
    revocationTime: Number(rawAttestation.revocationTime),
    recipient: rawAttestation.recipient,
    attester: rawAttestation.attester,
    revocable: rawAttestation.revocable,
    data: rawAttestation.decodedDataJson,
    chainId: chainId,
  };
}

/**
 * Fetches an attestation from the EAS GraphQL API by chain ID and attestation ID
 * @param chainId The chain ID where the attestation exists
 * @param attestationId The UID of the attestation
 * @returns A Promise that resolves to AttestationData
 */
async function fetchAttestations(chainId: number, attestationId: string): Promise<AttestationData> {
  // Map chainId to EAS subgraph endpoint
  const endpoint = getEASGraphQLEndpoint(chainId);
  if (!endpoint) {
    throw new Error(`Unsupported chainId: ${chainId}`);
  }

  console.log("attestationId", attestationId);

  const query = `query GetAttestation($uid: String!) {
      attestations(where: { refUID: {equals: $uid} }) {
        id
        schemaId
        refUID
        time
        expirationTime
        revocationTime
        recipient
        attester
        revocable
        decodedDataJson
      }
    }`;

  const headers = { "Content-Type": "application/json" };
  const body = JSON.stringify({
    query,
    variables: { uid: attestationId },
  });
  console.log("\nGraphQL query\n", query);
  console.log("\nGraphQL request headers:\n", headers);
  console.log("\nGraphQL request body:\n", body);

  const response = await fetch(endpoint, {
    method: "POST",
    headers,
    body,
  });

  if (!response.ok) {
    console.log('GraphQL error', response);
    throw new Error(`GraphQL error: ${response.status}`);
  }

  const { data } = await response.json();
  if (!data || !data.attestations || data.attestations.length === 0) {
    throw new Error(`No attestations found referencing attestationId: ${attestationId}`);
  }

  const firstAttestation = data.attestations[0];
  return convertRawAttestationToData(firstAttestation, chainId);
}

function getEASGraphQLEndpoint(chainId: number): string | null {
  // Add more chain IDs and endpoints as needed
  switch (chainId) {
    case 1:
      return "https://mainnet.easscan.org/graphql";
    case 10:
      return "https://optimism.easscan.org/graphql";
    case 11155111:
      return "https://sepolia.easscan.org/graphql";
    default:
      return null;
  }
}

// Convert AttestationData to JSON
function attestationToJson(attestation: AttestationData): string {
  try {
    return JSON.stringify(attestation);
  } catch (error) {
    throw new Error(
      `Failed to marshal JSON: ${
        error instanceof Error ? error.message : String(error)
      }`
    );
  }
}

/**
 * Extracts the location field from the decoded attestation data array
 * @param decodedData The decoded attestation data array
 * @returns The location value as a parsed JSON object, or null if not found
 */
function extractLocationFromAttestation(decodedData: any[]): any {
  if (!Array.isArray(decodedData)) {
    throw new Error("Decoded attestation data is not an array");
  }

  const locationField = decodedData.find(field => field.name === "location");
  if (!locationField) {
    throw new Error("No location field found in attestation data");
  }

  if (!locationField.value || !locationField.value.value) {
    throw new Error("Location field does not contain expected nested value structure");
  }

  try {
    // Parse the location value which is a JSON string containing coordinates
    return JSON.parse(locationField.value.value);
  } catch (error) {
    throw new Error(`Failed to parse location JSON: ${error instanceof Error ? error.message : String(error)}`);
  }
}

export { run };
