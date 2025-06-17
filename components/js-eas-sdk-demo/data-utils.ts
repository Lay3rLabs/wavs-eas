/**
 * Converts decoded attestation data array into a flat key-value object
 * @param decodedData The decoded attestation data array
 * @returns An object with field names as keys and their values
 */
export function extractDecodedAttestationData(decodedData: any[]): Record<string, any> {
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
export function getLocationUID(decodedData: any[]): string | null {
  try {
    const fields = extractDecodedAttestationData(decodedData);
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
export function getLocation(decodedData: any[]): any {
  const fields = extractDecodedAttestationData(decodedData);

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
