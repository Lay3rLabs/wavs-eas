mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction, WasmResponse};
use serde::{Deserialize, Serialize};

struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    /// Generic EAS attestation component.
    /// 
    /// This component receives attestation input data and creates a new EAS attestation.
    /// It can handle various input formats and automatically creates properly formatted
    /// attestation data for submission to the EAS contract.
    /// 
    /// Input formats supported:
    /// 1. JSON with schema, recipient, and data fields
    /// 2. Raw bytes data (will use default schema and recipient)
    /// 3. Structured attestation request data
    /// 
    /// The component will:
    /// 1. Parse the input attestation data
    /// 2. Validate the required fields
    /// 3. Format the data for EAS submission
    /// 4. Return the attestation request for blockchain submission
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // Parse the input attestation data
        let attestation_input = parse_attestation_input(&req)?;
        
        println!("Creating attestation: schema={}, recipient={}, data_len={}", 
                hex::encode(&attestation_input.schema),
                hex::encode(&attestation_input.recipient), 
                attestation_input.data.len());

        // Create the attestation response
        let attestation_response = AttestationResponse {
            schema: attestation_input.schema,
            recipient: attestation_input.recipient,
            data: attestation_input.data,
            expiration_time: attestation_input.expiration_time,
            revocable: attestation_input.revocable,
            ref_uid: attestation_input.ref_uid,
        };

        // Encode the response
        let encoded_response = serde_json::to_vec(&attestation_response)
            .map_err(|e| format!("Failed to encode attestation response: {}", e))?;

        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &encoded_response)),
            Destination::CliOutput => Some(WasmResponse { payload: encoded_response.into(), ordering: None }),
        };
        
        Ok(output)
    }
}

/// Parses input data into attestation input format
/// 
/// Supports multiple input formats:
/// 1. JSON string with structured attestation data
/// 2. Raw bytes data (uses defaults for schema/recipient)
/// 3. ABI-encoded attestation data
fn parse_attestation_input(data: &[u8]) -> Result<AttestationInput, String> {
    // Try to parse as JSON first
    if let Ok(json_str) = std::str::from_utf8(data) {
        if json_str.starts_with('{') {
            if let Ok(json_input) = serde_json::from_str::<JsonAttestationInput>(json_str) {
                return Ok(AttestationInput {
                    schema: hex_to_bytes32(&json_input.schema)?,
                    recipient: hex_to_address(&json_input.recipient)?,
                    data: json_input.data.into_bytes(),
                    expiration_time: json_input.expiration_time.unwrap_or(0),
                    revocable: json_input.revocable.unwrap_or(true),
                    ref_uid: if let Some(ref_uid) = json_input.ref_uid {
                        hex_to_bytes32(&ref_uid)?
                    } else {
                        [0u8; 32]
                    },
                });
            }
        }
    }

    // Try to decode as ABI-encoded attestation data
    if data.len() >= 84 { // Minimum: 32 bytes schema + 20 bytes recipient + 32 bytes for data length
        if let Ok(input) = try_decode_abi_attestation(data) {
            return Ok(input);
        }
    }

    // Fallback: treat as raw data with default schema and recipient
    Ok(AttestationInput {
        schema: [0u8; 32], // Default schema - should be configured
        recipient: [0u8; 20], // Zero address means no specific recipient
        data: data.to_vec(),
        expiration_time: 0, // No expiration
        revocable: true,
        ref_uid: [0u8; 32], // No reference
    })
}

/// Attempts to decode ABI-encoded attestation data
fn try_decode_abi_attestation(data: &[u8]) -> Result<AttestationInput, String> {
    // For now, skip complex ABI decoding and return an error
    // This can be implemented later if needed for specific use cases
    Err("ABI decoding not yet implemented".to_string())
}

/// Converts hex string to 32-byte array
fn hex_to_bytes32(hex: &str) -> Result<[u8; 32], String> {
    let hex = hex.strip_prefix("0x").unwrap_or(hex);
    if hex.len() != 64 {
        return Err(format!("Invalid hex length for bytes32: {}", hex.len()));
    }
    
    let mut bytes = [0u8; 32];
    for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let hex_str = std::str::from_utf8(chunk).map_err(|e| e.to_string())?;
        bytes[i] = u8::from_str_radix(hex_str, 16).map_err(|e| e.to_string())?;
    }
    Ok(bytes)
}

/// Converts hex string to 20-byte address
fn hex_to_address(hex: &str) -> Result<[u8; 20], String> {
    let hex = hex.strip_prefix("0x").unwrap_or(hex);
    if hex.len() != 40 {
        return Err(format!("Invalid hex length for address: {}", hex.len()));
    }
    
    let mut bytes = [0u8; 20];
    for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let hex_str = std::str::from_utf8(chunk).map_err(|e| e.to_string())?;
        bytes[i] = u8::from_str_radix(hex_str, 16).map_err(|e| e.to_string())?;
    }
    Ok(bytes)
}

/// JSON input format for attestations
#[derive(Debug, Deserialize)]
struct JsonAttestationInput {
    schema: String,           // Hex string
    recipient: String,        // Hex address
    data: String,            // String data to attest
    expiration_time: Option<u64>,
    revocable: Option<bool>,
    ref_uid: Option<String>, // Optional reference UID
}

/// Internal attestation input structure
#[derive(Debug)]
struct AttestationInput {
    schema: [u8; 32],
    recipient: [u8; 20],
    data: Vec<u8>,
    expiration_time: u64,
    revocable: bool,
    ref_uid: [u8; 32],
}

/// Attestation response structure for output
#[derive(Debug, Serialize)]
struct AttestationResponse {
    schema: [u8; 32],
    recipient: [u8; 20],
    data: Vec<u8>,
    expiration_time: u64,
    revocable: bool,
    ref_uid: [u8; 32],
}
