import { AttestationData } from "./types";
import { getLocationUID } from "./data-utils";

/**
 * Converts a raw attestation object from GraphQL response to AttestationData format
 * @param rawAttestation The raw attestation object from GraphQL
 * @returns AttestationData object
 */
export function convertRawAttestationToData(rawAttestation: any): AttestationData {
  return {
    uid: rawAttestation.id,
    schemaId: rawAttestation.schemaId,
    refUID: rawAttestation.refUID,
    data: rawAttestation.decodedDataJson,
  };
}

export class EASGraphQLClient {
  /**
   * Gets the GraphQL endpoint for a given chain ID
   * @param chainId The chain ID
   * @returns The GraphQL endpoint URL or null if unsupported
   */
  static getEndpoint(chainId: number): string | null {
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

  /**
   * Fetches the original attestation, its location attestation, and any attestations that reference it
   * @param chainId The chain ID where the attestations exist
   * @param attestationId The UID of the attestation to fetch and find references for
   * @returns A Promise that resolves to an object containing attestations array and location attestation
   */
  async fetchAttestations(chainId: number, attestationId: string): Promise<{
    attestations: AttestationData[];
    locationAttestation: AttestationData | null;
  }> {
    const endpoint = EASGraphQLClient.getEndpoint(chainId);
    if (!endpoint) {
      throw new Error(`Unsupported chainId: ${chainId}`);
    }

    console.log("attestationId", attestationId);

    // Step 1: Fetch the original attestation and any that reference it
    const initialData = await this.fetchInitialAttestations(endpoint, attestationId);

    const allAttestations: any[] = [];
    let locationUID: string | null = null;

    // Process the original attestation (only for extracting locationUID, not for final results)
    if (initialData.attestation) {
      // Extract locationUID from decodedDataJson
      try {
        const decodedData = JSON.parse(initialData.attestation.decodedDataJson);
        locationUID = getLocationUID(decodedData);
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
      const locationAttestation = await this.fetchLocationAttestation(endpoint, locationUID);
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
  private async executeGraphQLQuery(
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
  private async fetchInitialAttestations(endpoint: string, attestationId: string): Promise<any> {
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

    return this.executeGraphQLQuery(endpoint, query, { uid: attestationId }, true);
  }

  /**
   * Fetches a location attestation by its UID
   */
  private async fetchLocationAttestation(endpoint: string, locationUID: string): Promise<any | null> {
    const query = `query GetLocationAttestation($uid: String!) {
        attestation(where: { id: $uid }) {
          id
          schemaId
          refUID
          decodedDataJson
        }
      }`;

    const data = await this.executeGraphQLQuery(endpoint, query, { uid: locationUID }, false);
    return data?.attestation || null;
  }
}
