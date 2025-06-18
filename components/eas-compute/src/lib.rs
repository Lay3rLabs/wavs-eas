mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
pub mod bindings;
use crate::bindings::{export, host::get_evm_chain_config, Guest, TriggerAction, WasmResponse};
use alloy_network::Ethereum;
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types::{TransactionInput, TransactionRequest};
use alloy_sol_types::{sol, SolCall};
use serde::{Deserialize, Serialize};
use wavs_wasi_utils::evm::{
    alloy_primitives::{Address, FixedBytes},
    new_evm_provider,
};
use wstd::runtime::block_on;

sol! {
    interface IEAS {
        struct Attestation {
            bytes32 uid;
            bytes32 schema;
            uint64 time;
            uint64 expirationTime;
            uint64 revocationTime;
            bytes32 refUID;
            address recipient;
            address attester;
            bool revocable;
            bytes data;
        }

        function getAttestation(bytes32 uid) external view returns (Attestation memory);
    }
}

struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    /// EAS Compute Component - Queries chain for EAS attestation data
    ///
    /// This component queries the blockchain to fetch EAS attestation information
    /// and performs computations on attestation data.
    ///
    /// Input formats supported:
    /// 1. Attestation UID (32 bytes hex string)
    /// 2. JSON with attestation UID: {"uid": "0x..."}
    /// 3. Raw bytes representing the UID
    ///
    /// The component will:
    /// 1. Parse the input to extract attestation UID
    /// 2. Query the EAS contract to get attestation data
    /// 3. Process and analyze the attestation
    /// 4. Return structured attestation information
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // Parse the input to get attestation UID
        let attestation_uid = parse_attestation_uid(&req)?;

        println!("Querying EAS attestation: {}", hex::encode(attestation_uid));

        // Query the chain for attestation data
        let attestation_data =
            block_on(async move { query_eas_attestation(attestation_uid).await })?;

        println!("Retrieved attestation data: {:?}", attestation_data);

        // Encode the response
        let encoded_response = serde_json::to_vec(&attestation_data)
            .map_err(|e| format!("Failed to encode attestation data: {}", e))?;

        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &encoded_response)),
            Destination::CliOutput => {
                Some(WasmResponse { payload: encoded_response.into(), ordering: None })
            }
        };

        Ok(output)
    }
}

/// Parses input data to extract EAS attestation UID
fn parse_attestation_uid(data: &[u8]) -> Result<[u8; 32], String> {
    // Try to parse as JSON first
    if let Ok(json_str) = std::str::from_utf8(data) {
        if json_str.starts_with('{') {
            if let Ok(json_input) = serde_json::from_str::<JsonAttestationInput>(json_str) {
                return hex_to_bytes32(&json_input.uid);
            }
        }

        // Try to parse as hex string
        if json_str.starts_with("0x") || json_str.len() == 64 {
            return hex_to_bytes32(json_str);
        }
    }

    // Try to parse as raw 32 bytes
    if data.len() == 32 {
        let mut uid = [0u8; 32];
        uid.copy_from_slice(data);
        return Ok(uid);
    }

    Err("Unable to parse attestation UID from input data".to_string())
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

/// Queries the EAS contract to get attestation data
async fn query_eas_attestation(uid: [u8; 32]) -> Result<AttestationData, String> {
    // Get the chain configuration from WAVS
    let chain_config =
        get_evm_chain_config("local").ok_or("Failed to get chain config".to_string())?;

    // Create an Alloy provider instance
    let provider: RootProvider<Ethereum> = new_evm_provider::<Ethereum>(
        chain_config.http_endpoint.ok_or("No HTTP endpoint configured")?,
    );

    // EAS contract address (this would be configured or passed as parameter)
    // For now using a placeholder - in production this would come from config
    let eas_contract_address = Address::from([0u8; 20]); // Replace with actual EAS address

    // Create the getAttestation function call
    let call_data = IEAS::getAttestationCall { uid: FixedBytes(uid) };

    // Prepare the transaction request
    let tx_request = TransactionRequest {
        to: Some(eas_contract_address.into()),
        input: TransactionInput::new(call_data.abi_encode().into()),
        ..Default::default()
    };

    // Execute the call
    let result = provider
        .call(tx_request)
        .await
        .map_err(|e| format!("Failed to call EAS contract: {}", e))?;

    // Decode the result
    let return_data = IEAS::getAttestationCall::abi_decode_returns(&result)
        .map_err(|e| format!("Failed to decode attestation result: {}", e))?;

    // Convert to our data structure
    let att = return_data;
    Ok(AttestationData {
        uid,
        schema: att.schema.0,
        time: att.time,
        expiration_time: att.expirationTime,
        revocation_time: att.revocationTime,
        ref_uid: att.refUID.0,
        recipient: att.recipient.into(),
        attester: att.attester.into(),
        revocable: att.revocable,
        data: att.data.to_vec(),
        // Computed fields
        is_valid: att.revocationTime == 0
            && (att.expirationTime == 0 || get_current_timestamp() < att.expirationTime),
        age_seconds: get_current_timestamp().saturating_sub(att.time),
        data_length: att.data.len(),
        has_recipient: att.recipient != Address::ZERO,
        has_reference: att.refUID != FixedBytes([0u8; 32]),
    })
}

/// Gets current timestamp
fn get_current_timestamp() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

/// JSON input format for attestation UID
#[derive(Debug, Deserialize)]
struct JsonAttestationInput {
    uid: String, // Hex string
}

/// Comprehensive attestation data structure
#[derive(Debug, Serialize)]
struct AttestationData {
    // Core EAS fields
    uid: [u8; 32],
    schema: [u8; 32],
    time: u64,
    expiration_time: u64,
    revocation_time: u64,
    ref_uid: [u8; 32],
    recipient: [u8; 20],
    attester: [u8; 20],
    revocable: bool,
    data: Vec<u8>,

    // Computed fields for analysis
    is_valid: bool,      // Not revoked and not expired
    age_seconds: u64,    // Age of attestation in seconds
    data_length: usize,  // Length of attestation data
    has_recipient: bool, // Whether it has a specific recipient
    has_reference: bool, // Whether it references another attestation
}
