import { TriggerAction, WasmResponse } from "./out/wavs:worker@0.4.0-beta.4";
import { decodeTriggerEvent, encodeOutput, Destination } from "./trigger";
import { ethers } from "ethers";
import { AbiCoder } from "ethers";
import { hexlify } from "ethers";
import booleanContains from "@turf/boolean-contains";
import { point, polygon } from "@turf/helpers";

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

  const { attestations, locationAttestation } = await fetchAttestations(Number(chainId), attestationId);
  console.log(`\nreceived ${attestations.length} attestations\n`);

  if (!locationAttestation) {
    throw new Error("No location attestation found for polygon containment testing");
  }

  // Extract polygon from location attestation
  const locationAttestationJson = JSON.parse(locationAttestation.data);
  const polygonData = extractLocationFromAttestation(locationAttestationJson);
  console.log(`\npolygon data from location attestation\n`, JSON.stringify(polygonData, null, 2));

  // Create turf polygon from the location attestation
  const turfPolygon = polygon(polygonData.coordinates);

  const containmentResults = attestations.map((attestation, index) => {
    const attestationJson = JSON.parse(attestation.data);
    console.log(`\ndecoded attestation ${index + 1} data\n`, JSON.stringify(attestationJson, null, 2));

    const locationData = extractLocationFromAttestation(attestationJson);
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

// ======================== EAS GraphQL ========================

interface AttestationData {
  uid: string;
  schemaId: string;
  refUID: string;
  data: string;
}

/**
 * Converts a raw attestation object from GraphQL response to AttestationData format
 * @param rawAttestation The raw attestation object from GraphQL
 * @returns AttestationData object
 */
function convertRawAttestationToData(rawAttestation: any): AttestationData {
  return {
    uid: rawAttestation.id,
    schemaId: rawAttestation.schemaId,
    refUID: rawAttestation.refUID,
    data: rawAttestation.decodedDataJson,
  };
}

/**
 * Fetches the original attestation, its location attestation, and any attestations that reference it
 * @param chainId The chain ID where the attestations exist
 * @param attestationId The UID of the attestation to fetch and find references for
 * @returns A Promise that resolves to an object containing attestations array and location attestation
 */
async function fetchAttestations(chainId: number, attestationId: string): Promise<{
  attestations: AttestationData[];
  locationAttestation: AttestationData | null;
}> {
  const endpoint = getEASGraphQLEndpoint(chainId);
  if (!endpoint) {
    throw new Error(`Unsupported chainId: ${chainId}`);
  }

  console.log("attestationId", attestationId);

  // Step 1: Fetch the original attestation and any that reference it
  const initialData = await fetchInitialAttestations(endpoint, attestationId);

  const allAttestations: any[] = [];
  let locationUID: string | null = null;

    // Process the original attestation (only for extracting locationUID, not for final results)
  if (initialData.attestation) {
    // Extract locationUID from decodedDataJson
    try {
      const decodedData = JSON.parse(initialData.attestation.decodedDataJson);
      locationUID = extractLocationUIDFromAttestation(decodedData);
      console.log(`\nFound original attestation: ${initialData.attestation.id} (used for locationUID extraction only)`);
      if (locationUID) {
        console.log(`\nExtracted locationUID: ${locationUID}`);
      }
    } catch (error) {
      console.log(`\nWarning: Could not parse decodedDataJson for original attestation: ${error}`);
    }
  }

  // Add referencing attestations
  if (initialData.attestations && initialData.attestations.length > 0) {
    allAttestations.push(...initialData.attestations);
    console.log(`\nFound ${initialData.attestations.length} attestations referencing: ${attestationId}`);
  }

  // Step 2: If we have a locationUID, fetch that attestation too
  let locationAttestationData: AttestationData | null = null;
  if (locationUID) {
    const locationAttestation = await fetchLocationAttestation(endpoint, locationUID);
    if (locationAttestation) {
      locationAttestationData = convertRawAttestationToData(locationAttestation);
      console.log(`\nFound location attestation: ${locationAttestation.id}`);
    }
  }

  // Ensure we have at least one attestation
  if (allAttestations.length === 0) {
    throw new Error(`No attestations found for attestationId: ${attestationId}`);
  }

  console.log(`\nTotal attestations to process: ${allAttestations.length}`);

  return {
    attestations: allAttestations.map((attestation: any) => convertRawAttestationToData(attestation)),
    locationAttestation: locationAttestationData
  };
}

/**
 * Generic function to execute GraphQL queries
 * @param endpoint The GraphQL endpoint URL
 * @param query The GraphQL query string
 * @param variables Variables for the query
 * @param throwOnError Whether to throw on HTTP errors or return null
 * @returns The data from the GraphQL response or null if error and throwOnError is false
 */
async function executeGraphQLQuery(
  endpoint: string,
  query: string,
  variables: Record<string, any>,
  throwOnError: boolean = true
): Promise<any> {
  const response = await fetch(endpoint, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ query, variables }),
  });

  if (!response.ok) {
    const errorMessage = `GraphQL error: ${response.status}`;
    if (throwOnError) {
      console.log('GraphQL error', response);
      throw new Error(errorMessage);
    } else {
      console.log(`Warning: ${errorMessage}`);
      return null;
    }
  }

  const { data } = await response.json();
  if (!data && throwOnError) {
    throw new Error(`No data returned from GraphQL query`);
  }

  return data;
}

/**
 * Fetches the initial attestation and any that reference it
 */
async function fetchInitialAttestations(endpoint: string, attestationId: string): Promise<any> {
  const query = `query GetAttestation($uid: String!) {
      attestation(where: { id: $uid }) {
        id
        schemaId
        refUID
        decodedDataJson
      }

      attestations(where: { refUID: {equals: $uid} }) {
        id
        schemaId
        refUID
        decodedDataJson
      }
    }`;

  return executeGraphQLQuery(endpoint, query, { uid: attestationId }, true);
}

/**
 * Fetches a location attestation by its UID
 */
async function fetchLocationAttestation(endpoint: string, locationUID: string): Promise<any | null> {
  const query = `query GetLocationAttestation($uid: String!) {
      attestation(where: { id: $uid }) {
        id
        schemaId
        refUID
        decodedDataJson
      }
    }`;

  const data = await executeGraphQLQuery(endpoint, query, { uid: locationUID }, false);
  return data?.attestation || null;
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
 * Converts decoded attestation data array into a flat key-value object
 * @param decodedData The decoded attestation data array
 * @returns An object with field names as keys and their values
 */
function extractAllFieldsFromAttestation(decodedData: any[]): Record<string, any> {
  if (!Array.isArray(decodedData)) {
    throw new Error("Decoded attestation data is not an array");
  }

  const fields: Record<string, any> = {};

  for (const field of decodedData) {
    if (!field.name || !field.value) {
      console.log(`Warning: Skipping malformed field:`, field);
      continue;
    }

    // Extract the actual value, handling different value structures
    let value = field.value.value;

    // Handle different data types
    if (field.value.type === "uint256" && value?.type === "BigNumber") {
      // Convert BigNumber to regular number
      value = parseInt(value.hex, 16);
    }

    fields[field.name] = value;
  }

  return fields;
}

/**
 * Extracts the locationUID field from the decoded attestation data
 * @param decodedData The decoded attestation data array
 * @returns The locationUID value as a string, or null if not found
 */
function extractLocationUIDFromAttestation(decodedData: any[]): string | null {
  try {
    const fields = extractAllFieldsFromAttestation(decodedData);
    return fields.locationUID || null;
  } catch (error) {
    console.log(`Warning: Could not extract locationUID: ${error}`);
    return null;
  }
}

/**
 * Extracts the location field from the decoded attestation data
 * @param decodedData The decoded attestation data array
 * @returns The location value as a parsed JSON object
 */
function extractLocationFromAttestation(decodedData: any[]): any {
  const fields = extractAllFieldsFromAttestation(decodedData);

  if (!fields.location) {
    throw new Error("No location field found in attestation data");
  }

  try {
    // Parse the location value which is a JSON string containing coordinates
    return JSON.parse(fields.location);
  } catch (error) {
    throw new Error(`Failed to parse location JSON: ${error instanceof Error ? error.message : String(error)}`);
  }
}

export { run };
