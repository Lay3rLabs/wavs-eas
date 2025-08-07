use crate::bindings::wavs::worker::layer_types::{
    TriggerData, TriggerDataEvmContractEvent, WasmResponse,
};
use alloy_primitives::{FixedBytes, U256};
use alloy_sol_types::SolType;
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
pub fn decode_trigger_event(
    trigger_data: TriggerData,
) -> Result<(AttestationRequest, Destination)> {
    match trigger_data {
        TriggerData::EvmContractEvent(TriggerDataEvmContractEvent { log, .. }) => {
            // Decode the AttestationRequested event
            let event: AttestationRequested = decode_event_log_data!(log)?;
            // Create attestation request data from the event
            let attestation_request = AttestationRequest {
                schema: event.schema,
                data: AttestationRequestData {
                    recipient: event.recipient,
                    expirationTime: 0, // NO_EXPIRATION_TIME
                    revocable: true,
                    refUID: FixedBytes::<32>::ZERO, // EMPTY_UID
                    data: event.data.into(),
                    value: U256::ZERO,
                },
            };
            return Ok((attestation_request, Destination::Ethereum));
        }
        TriggerData::Raw(data) => {
            Ok((AttestationRequest::abi_decode(&data)?, Destination::CliOutput))
        }
        _ => Err(anyhow::anyhow!("Unsupported trigger data type")),
    }
}

/// Encodes the attestation output data for submission back to Ethereum
///
/// # Arguments
/// * `output` - The attestation data to be encoded
///
/// # Returns
/// ABI encoded bytes ready for submission to Ethereum
pub fn encode_trigger_output(output: impl AsRef<[u8]>) -> WasmResponse {
    WasmResponse { payload: output.as_ref().to_vec().into(), ordering: None }
}

/// Solidity type definitions for EAS attestation processing
///
/// Minimal types needed for this component, defined inline for simplicity.
/// Focuses on direct EAS event processing without redundant trigger wrappers.
use alloy_sol_macro::sol;

sol! {
    /// Event emitted when an attestation is requested
    event AttestationRequested(
        address indexed creator,
        bytes32 indexed schema,
        address indexed recipient,
        bytes data
    );

    /// @notice A struct representing the arguments of the attestation request.
    struct AttestationRequestData {
        address recipient; // The recipient of the attestation.
        uint64 expirationTime; // The time when the attestation expires (Unix timestamp).
        bool revocable; // Whether the attestation is revocable.
        bytes32 refUID; // The UID of the related attestation.
        bytes data; // Custom attestation data.
        uint256 value; // An explicit ETH amount to send to the resolver. This is important to prevent accidental user errors.
    }

    /// @notice A struct representing the full arguments of the attestation request.
    struct AttestationRequest {
        bytes32 schema; // The unique identifier of the schema.
        AttestationRequestData data; // The arguments of the attestation request.
    }
}
