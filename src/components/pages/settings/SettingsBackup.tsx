import { invoke } from "@tauri-apps/api";
import { BsGlobeAmericas } from "solid-icons/bs";
import { FiUpload } from "solid-icons/fi";
import { RiDeviceSave3Fill } from "solid-icons/ri";
import { TbDatabaseStar } from "solid-icons/tb";
import { Component, Show, createEffect, createSignal, on } from "solid-js";
import SettingsStore from "../../../store/SettingsStore";
import SwitchField from "../../elements/SwitchField";
import { TextBlock } from "../../elements/TextBlock";

interface SettingsBackupProps {}

export const SettingsBackup: Component<SettingsBackupProps> = ({}) => {
  const [url, setUrl] = createSignal<string>();
  const { settings, syncClipboard } = SettingsStore;

  createEffect(
    on(
      () => settings()?.synchronize,
      () =>
        setTimeout(
          async () => setUrl(await invoke<string>("get_db_path")),
          100,
        ),
    ),
  );

  return (
    <>
      <TextBlock Icon={TbDatabaseStar} title="Sync">
        <div class="mb-2 flex items-center justify-between space-x-2 px-5 pb-2.5">
          <div class="flex items-center space-x-2 truncate">
            <RiDeviceSave3Fill />
            <h6 class="text-sm">Synchronize clipboard history</h6>
          </div>
          <div>
            <SwitchField
              checked={settings()?.synchronize || false}
              onChange={() => syncClipboard()}
            />
          </div>
        </div>
      </TextBlock>

      <Show when={url()}>
        <TextBlock
          Icon={BsGlobeAmericas}
          title="Database Location"
          className="animate-fade"
        >
          <div class="list-disc px-5 pb-5 pt-2.5">
            <button
              type="button"
              class="group relative w-full cursor-pointer"
              onClick={syncClipboard}
            >
              <div
                title={url()}
                class="w-full truncate rounded-md border border-gray-300 px-3 py-0.5 text-left text-sm italic focus:outline-none dark:border-dark-light dark:bg-dark-light dark:text-white dark:focus:bg-dark-dark"
              >
                {url()}
              </div>
              <div class="group absolute inset-y-0 right-1 my-1 flex items-center space-x-1 rounded bg-gray-600 px-2 text-xs text-white group-hover:bg-gray-400">
                <FiUpload class="dark:text-white" />
                <div>Browse</div>
              </div>
            </button>
          </div>
        </TextBlock>
      </Show>
    </>
  );
};
