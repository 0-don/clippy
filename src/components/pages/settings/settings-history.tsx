import { BsDeviceHdd } from "solid-icons/bs";
import { FiTrash2 } from "solid-icons/fi";
import { Component, createSignal, onMount } from "solid-js";
import { DatabaseInfo } from "../../../types";
import { InvokeCommand } from "../../../types/tauri-invoke";
import { formatBytes } from "../../../utils/helpers";
import { invokeCommand } from "../../../utils/tauri";
import { TextBlock } from "../../elements/text-block";

interface SettingsHistoryProps {}

export const SettingsHistory: Component<SettingsHistoryProps> = ({}) => {
  const [databaseInfo, setDatabaseInfo] = createSignal<DatabaseInfo>({
    records: 0,
    size: 0,
  });

  onMount(async () => setDatabaseInfo(await invokeCommand(InvokeCommand.GetDbInfo)));

  return (
    <>
      <TextBlock Icon={BsDeviceHdd} title="Local Storage">
        <ul class="mx-5 list-disc px-5 pb-5">
          <li>{`${databaseInfo().records} local items (${formatBytes(
            databaseInfo().size
          )}) are saved on this computer`}</li>
        </ul>
      </TextBlock>

      <div class="flex w-full justify-end">
        <button
          type="button"
          onClick={async () => {
            await invokeCommand(InvokeCommand.ClearClipboards);
            setDatabaseInfo(await invokeCommand(InvokeCommand.GetDbInfo));
          }}
          class="inline-flex items-center space-x-2 rounded bg-zinc-600 px-4 py-2 text-sm font-bold text-white hover:bg-zinc-700"
        >
          <FiTrash2 />
          <span>Clear...</span>
        </button>
      </div>
    </>
  );
};
