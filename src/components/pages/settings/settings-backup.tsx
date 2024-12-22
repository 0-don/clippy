import { AiTwotoneFolderOpen } from "solid-icons/ai";
import { BsGearWideConnected } from "solid-icons/bs";
import { RiDeviceSave3Fill } from "solid-icons/ri";
import { SiSqlite } from "solid-icons/si";
import { TbDatabaseStar } from "solid-icons/tb";
import { Component, createResource } from "solid-js";
import { SettingsStore } from "../../../store/settings-store";
import { FolderLocation } from "../../../types/enums";
import { InvokeCommand } from "../../../types/tauri-invoke";
import { invokeCommand } from "../../../utils/tauri";
import { TextBlock } from "../../elements/text-block";
import { Toggle } from "../../elements/toggle";

interface SettingsBackupProps {}

export const SettingsBackup: Component<SettingsBackupProps> = ({}) => {
  const [databaseUrl, setDatabaseUrl] = createResource(() => invokeCommand(InvokeCommand.GetDbPath));
  const [configUrl] = createResource(() => invokeCommand(InvokeCommand.GetConfigPath));

  return (
    <>
      <TextBlock Icon={TbDatabaseStar} title="Sync">
        <div class="mb-2 flex items-center justify-between space-x-2 px-5 pb-2.5">
          <div class="flex items-center space-x-2 truncate">
            <RiDeviceSave3Fill />
            <h6 class="text-sm">Synchronize clipboard history</h6>
          </div>

          <Toggle
            checked={!!SettingsStore.settings()?.synchronize}
            onChange={async () => {
              await SettingsStore.syncClipboard();
              setDatabaseUrl.refetch();
            }}
          />
        </div>
      </TextBlock>

      <TextBlock Icon={SiSqlite} title="Database Location">
        <div class="list-disc px-5 pb-5 pt-2.5">
          <div class="relative w-full cursor-pointer">
            <div
              title={databaseUrl()}
              class="w-full truncate rounded-md border border-gray-300 px-3 py-0.5 text-left text-sm italic focus:outline-none dark:border-dark-light dark:bg-dark-light dark:text-white dark:focus:bg-dark-dark"
            >
              {databaseUrl()}
            </div>
            <button
              type="button"
              onClick={() => invokeCommand(InvokeCommand.OpenFolder, { location: FolderLocation.Database })}
              class="group absolute inset-y-0 right-1 my-1 flex items-center space-x-1 rounded bg-gray-600 px-2 text-xs text-white group-hover:bg-gray-400"
            >
              <AiTwotoneFolderOpen class="dark:text-white" />
              <div>Open</div>
            </button>
          </div>
        </div>
      </TextBlock>

      <TextBlock Icon={BsGearWideConnected} title="Config Location">
        <div class="list-disc px-5 pb-5 pt-2.5">
          <div class="relative w-full cursor-pointer">
            <div
              title={configUrl()}
              class="w-full truncate rounded-md border border-gray-300 px-3 py-0.5 text-left text-sm italic focus:outline-none dark:border-dark-light dark:bg-dark-light dark:text-white dark:focus:bg-dark-dark"
            >
              {configUrl()}
            </div>
            <button
              type="button"
              onClick={() => invokeCommand(InvokeCommand.OpenFolder, { location: FolderLocation.Config })}
              class="group absolute inset-y-0 right-1 my-1 flex items-center space-x-1 rounded bg-gray-600 px-2 text-xs text-white group-hover:bg-gray-400"
            >
              <AiTwotoneFolderOpen class="dark:text-white" />
              <div>Open</div>
            </button>
          </div>
        </div>
      </TextBlock>
    </>
  );
};
