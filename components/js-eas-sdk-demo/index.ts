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

  const attestation = await fetchAttestation(Number(chainId), attestationId);
  console.log("\nreceived attestation\n", JSON.stringify(attestation, null, 2));
  const attestationJson = attestationToJson(attestation);

  return new TextEncoder().encode(attestationJson);
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
 * Fetches an attestation from the EAS GraphQL API by chain ID and attestation ID
 * @param chainId The chain ID where the attestation exists
 * @param attestationId The UID of the attestation
 * @returns A Promise that resolves to AttestationData
 */
async function fetchAttestation(chainId: number, attestationId: string): Promise<AttestationData> {
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
        data
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
  if (!data || !data.attestations) {
    throw new Error(`Attestation not found: ${attestationId}`);
  }

  const att = data.attestations[0];
  return {
    uid: att.id,
    schemaId: att.schemaId,
    refUID: att.refUID,
    time: Number(att.time),
    expirationTime: Number(att.expirationTime),
    revocationTime: Number(att.revocationTime),
    recipient: att.recipient,
    attester: att.attester,
    revocable: att.revocable,
    data: att.data,
    chainId: chainId,
  };
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

export { run };
