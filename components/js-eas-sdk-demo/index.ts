import { TriggerAction, WasmResponse } from "./out/wavs:worker@0.4.0-beta.4";
import { decodeTriggerEvent, encodeOutput, Destination } from "./trigger";
import { compute } from "./compute";

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

export { run };
