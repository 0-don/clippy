import { AiTwotoneFolderOpen } from "solid-icons/ai";
import { BiRegularReset } from "solid-icons/bi";
import { BsGearWideConnected } from "solid-icons/bs";
import { SiSqlite } from "solid-icons/si";
import { TbDatabaseStar, TbExchange, TbNumber } from "solid-icons/tb";
import { Component, createResource, Show } from "solid-js";
import { invokeCommand } from "../../../lib/tauri";
import { HotkeyStore } from "../../../store/hotkey-store";
import { SettingsStore } from "../../../store/settings-store";
import { FolderLocation, HotkeyEvent } from "../../../types/enums";
import { InvokeCommand } from "../../../types/tauri-invoke";
import { Input } from "../../elements/input";
import { TextBlock } from "../../elements/text-block";
import { Toggle } from "../../elements/toggle";
import { useLanguage } from "../../provider/language-provider";

interface SettingsBackupProps {}

export const SettingsBackup: Component<SettingsBackupProps> = ({}) => {
  const { t } = useLanguage();
  const [databaseUrl, setDatabaseUrl] = createResource(() => invokeCommand(InvokeCommand.GetDbPath));
  const [configUrl] = createResource(() => invokeCommand(InvokeCommand.GetConfigPath));

  return (
    <>
      <TextBlock Icon={TbDatabaseStar} title={t("SETTINGS.BACKUP.SYNC")}>
        <div class="mb-2 flex items-center justify-between space-x-2 px-5 pb-2.5">
          <div class="flex items-center space-x-2 truncate">
            <div innerHTML={HotkeyStore.getHotkeyIcon(HotkeyEvent.SyncClipboardHistory)} class="relative" />
            <h6 class="text-sm">{t("SETTINGS.BACKUP.SYNCHRONIZE_CLIPBOARD_HISTORY")}</h6>
          </div>

          <Toggle
            checked={!!SettingsStore.settings()?.sync}
            onClick={
              !SettingsStore.settings()?.sync
                ? async () => void (await SettingsStore.syncAuthenticateToggle())
                : undefined
            }
            onChange={async () => void (await SettingsStore.syncAuthenticateToggle())}
          />
        </div>

        <Show when={SettingsStore.settings()?.sync}>
          <div class="flex items-center justify-between space-x-2 px-5 pb-5">
            <div class="flex items-center space-x-2 truncate">
              <TbNumber />
              <h6 class="text-sm">{t("SETTINGS.BACKUP.SYNC_LIMIT")}</h6>
            </div>

            <Input
              type="number"
              min={0}
              max={1000}
              value={SettingsStore.settings()!.sync_limit}
              debounce={1000}
              onInput={async (e) => {
                const settings = await invokeCommand(InvokeCommand.SyncLimitChange, {
                  syncLimit: Number(e.target.value),
                });
                SettingsStore.setSettings(settings);
              }}
            />
          </div>
        </Show>
      </TextBlock>

      <TextBlock Icon={SiSqlite} title={t("SETTINGS.BACKUP.DATABASE_LOCATION")}>
        <div class="list-disc px-5 pb-5 pt-2.5">
          <div class="relative w-full cursor-pointer">
            <div
              title={databaseUrl()}
              class="w-full truncate rounded-md border border-gray-300 px-3 py-0.5 text-left text-sm italic focus:outline-none dark:border-dark-light dark:bg-dark-light dark:text-white dark:focus:bg-dark-dark"
            >
              {databaseUrl()}
            </div>
          </div>
          <div class="mt-1 flex items-center justify-end gap-1">
            <button
              type="button"
              onClick={async () => {
                await SettingsStore.resetClipboardDbLocation();
                setDatabaseUrl.refetch();
              }}
              class="group my-1 flex items-center space-x-1 rounded bg-gray-600 px-2 text-xs text-white group-hover:bg-gray-400"
            >
              <BiRegularReset class="dark:text-white" />
              <div>{t("SETTINGS.BACKUP.RESET_LOCATION")}</div>
            </button>

            <button
              type="button"
              onClick={async () => {
                await SettingsStore.changeClipboardDbLocation();
                setDatabaseUrl.refetch();
              }}
              class="group my-1 flex items-center space-x-1 rounded bg-gray-600 px-2 text-xs text-white group-hover:bg-gray-400"
            >
              <TbExchange class="dark:text-white" />
              <div>{t("SETTINGS.BACKUP.CHANGE_LOCATION")}</div>
            </button>

            <button
              type="button"
              onClick={() => invokeCommand(InvokeCommand.OpenFolder, { location: FolderLocation.Database })}
              class="group my-1 flex items-center space-x-1 rounded bg-gray-600 px-2 text-xs text-white group-hover:bg-gray-400"
            >
              <AiTwotoneFolderOpen class="dark:text-white" />
              <div>{t("SETTINGS.BACKUP.OPEN")}</div>
            </button>
          </div>
        </div>
      </TextBlock>

      <TextBlock Icon={BsGearWideConnected} title={t("SETTINGS.BACKUP.CONFIG_LOCATION")}>
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
              <div>{t("SETTINGS.BACKUP.OPEN")}</div>
            </button>
          </div>
        </div>
      </TextBlock>
    </>
  );
};
