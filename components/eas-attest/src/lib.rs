mod trigger;
use alloy_sol_types::SolValue;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction, WasmResponse};

struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    /// Generic EAS attestation component.
    ///
    /// A generic component that receives attestation input data and creates a new EAS attestation.
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        let (req, dest) = decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        println!("Creating attestation: schema={}, recipient={}, expirationTime={}, revocable={}, refUID={}, data={}, value={}", req.schema, req.data.recipient, req.data.expirationTime, req.data.revocable, req.data.refUID, req.data.data, req.data.value);

        // ABI encode the attestation request
        let encoded_response = req.abi_encode();

        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(&encoded_response)),
            Destination::CliOutput => {
                Some(WasmResponse { payload: encoded_response.into(), ordering: None })
            }
        };

        Ok(output)
    }
}
