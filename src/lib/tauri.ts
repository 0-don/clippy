import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { TauriInvokeCommands } from "../types/tauri-invoke";
import { TauriListenEvents } from "../types/tauri-listen";

export function listenEvent<EventName extends keyof TauriListenEvents>(
  event: EventName,
  handler: (payload: TauriListenEvents[EventName]) => void
) {
  return listen(event, (event) => {
    handler(event.payload as TauriListenEvents[EventName]);
  });
}

export function invokeCommand<Command extends keyof TauriInvokeCommands>(
  command: Command,
  args?: TauriInvokeCommands[Command]["args"]
): Promise<TauriInvokeCommands[Command]["return"]> {
  return invoke(command, args);
}
