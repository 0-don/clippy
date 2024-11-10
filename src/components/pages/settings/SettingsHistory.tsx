import { invoke } from "@tauri-apps/api/core";
import { BsDeviceHdd } from "solid-icons/bs";
import { FiTrash2 } from "solid-icons/fi";
import { Component, createSignal, onMount } from "solid-js";
import { formatBytes } from "../../../utils/helpers";
import { TextBlock } from "../../elements/TextBlock";

interface SettingsHistoryProps {}

type DatabaseInfo = {
  records: number;
  size: number;
};

export const SettingsHistory: Component<SettingsHistoryProps> = ({}) => {
  const [databaseInfo, setDatabaseInfo] = createSignal<DatabaseInfo>({
    records: 0,
    size: 0,
  });

  onMount(async () => setDatabaseInfo(await invoke<DatabaseInfo>("get_db_size")));

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
            await invoke("clear_clipboards");
            setDatabaseInfo(await invoke<DatabaseInfo>("get_db_size"));
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
