use crate::bindings::wavs::worker::layer_types::{
    TriggerData, TriggerDataEvmContractEvent, WasmResponse,
};
use alloy_sol_types::SolValue;
use anyhow::Result;
use wavs_wasi_utils::decode_event_log_data;

/// Represents the destination where the trigger output should be sent
pub enum Destination {
    /// Output will be ABI encoded and sent to an Ethereum contract
    Ethereum,
    /// Raw output for local testing/debugging
    CliOutput,
}

/// Decodes incoming trigger event data into its components
///
/// # Arguments
/// * `trigger_data` - The raw trigger data received from WAVS
///
/// # Returns
/// A tuple containing:
/// * `u64` - Trigger ID for tracking the request (0 for direct events)
/// * `Vec<u8>` - The actual attestation data payload
/// * `Destination` - Where the processed result should be sent
pub fn decode_trigger_event(trigger_data: TriggerData) -> Result<(u64, Vec<u8>, Destination)> {
    match trigger_data {
        TriggerData::EvmContractEvent(TriggerDataEvmContractEvent { log, .. }) => {
            // Try to decode as EAS Attested event for direct attestation input
            let event: AttestedEvent = decode_event_log_data!(log)?;
            // Create attestation data from EAS event
            let attestation_data = AttestationEventData {
                uid: event.uid,
                schema: event.schema_uid,
                recipient: event.recipient,
                attester: event.attester,
            };
            let encoded_data = attestation_data.abi_encode();
            return Ok((0, encoded_data, Destination::Ethereum));
        }
        TriggerData::Raw(data) => Ok((0, data.clone(), Destination::CliOutput)),
        _ => Err(anyhow::anyhow!("Unsupported trigger data type")),
    }
}

/// Encodes the attestation output data for submission back to Ethereum
///
/// # Arguments
/// * `trigger_id` - The ID of the original trigger request
/// * `output` - The attestation data to be encoded
///
/// # Returns
/// ABI encoded bytes ready for submission to Ethereum
pub fn encode_trigger_output(trigger_id: u64, output: impl AsRef<[u8]>) -> WasmResponse {
    WasmResponse {
        payload: DataWithId { trigger_id, data: output.as_ref().to_vec().into() }.abi_encode(),
        ordering: None,
    }
}

/// Solidity type definitions for EAS attestation processing
///
/// Minimal types needed for this component, defined inline for simplicity.
/// Focuses on direct EAS event processing without redundant trigger wrappers.
use alloy_sol_macro::sol;

sol! {
    /// EAS Attested event - emitted when an attestation is made
    event AttestedEvent(
        address indexed recipient,
        address indexed attester,
        bytes32 uid,
        bytes32 indexed schema_uid
    );

    /// Response data structure with trigger ID
    struct DataWithId {
        uint64 trigger_id;
        bytes data;
    }

    /// Attestation event data for processing
    struct AttestationEventData {
        bytes32 uid;
        bytes32 schema;
        address recipient;
        address attester;
    }
}
