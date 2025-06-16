import { TriggerAction, WasmResponse } from "./out/wavs:worker@0.4.0-beta.4";
import { decodeTriggerEvent, encodeOutput, Destination } from "./trigger";
import { EAS, SchemaEncoder } from "@ethereum-attestation-service/eas-sdk";
import { ethers } from "ethers";

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
  const inputStr = new TextDecoder().decode(input);
  const { chainId, attestationId }: AttestationInput = JSON.parse(inputStr);

  const attestation = await fetchAttestation(chainId, attestationId);
  const attestationJson = attestationToJson(attestation);

  return new TextEncoder().encode(attestationJson);
}

// ======================== EAS ========================

interface AttestationData {
  uid: string;
  schema: string;
  refUID: string;
  time: bigint;
  expirationTime: bigint;
  revocationTime: bigint;
  recipient: string;
  attester: string;
  revocable: boolean;
  data: string;
  chainId: number;
}

/**
 * Fetches an attestation from the EAS API by chain ID and attestation ID
 * @param chainId The chain ID where the attestation exists
 * @param attestationId The UID of the attestation
 * @returns A Promise that resolves to AttestationData
 */
async function fetchAttestation(chainId: number, attestationId: string): Promise<AttestationData> {
  // Initialize EAS SDK
  const eas = new EAS("https://easscan.org/graphql");

  try {
    // Fetch the attestation
    const attestation = await eas.getAttestation(attestationId);

    if (!attestation) {
      throw new Error(`Attestation not found: ${attestationId}`);
    }

    return {
      uid: attestation.uid,
      schema: attestation.schema,
      refUID: attestation.refUID,
      time: attestation.time,
      expirationTime: attestation.expirationTime,
      revocationTime: attestation.revocationTime,
      recipient: attestation.recipient,
      attester: attestation.attester,
      revocable: attestation.revocable,
      data: attestation.data,
      chainId: chainId
    };
  } catch (error) {
    throw new Error(
      `Failed to fetch attestation: ${
        error instanceof Error ? error.message : String(error)
      }`
    );
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
