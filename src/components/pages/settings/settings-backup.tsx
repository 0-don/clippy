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
import { MAX_SYNC_LIMIT } from "../../../utils/constants";
import { Button } from "../../elements/button";
import { Input } from "../../elements/input";
import { TextBlock } from "../../elements/text-block";
import { Toggle } from "../../elements/toggle";
import { useLanguage } from "../../provider/language-provider";

interface SettingsBackupProps {}

export const SettingsBackup: Component<SettingsBackupProps> = ({}) => {
  const { t } = useLanguage();
  const [databaseUrl, setDatabaseUrl] = createResource(() =>
    invokeCommand(InvokeCommand.GetDbPath),
  );
  const [configUrl] = createResource(() =>
    invokeCommand(InvokeCommand.GetConfigPath),
  );

  return (
    <>
      <TextBlock Icon={TbDatabaseStar} title={t("SETTINGS.BACKUP.SYNC")}>
        <div class="mb-2 flex items-center justify-between space-x-2 px-5 pb-2.5">
          <div class="flex items-center space-x-2 truncate">
            <div
              innerHTML={HotkeyStore.getHotkeyIcon(
                HotkeyEvent.SyncClipboardHistory,
              )}
              class="relative"
            />
            <h6 class="text-sm">
              {t("SETTINGS.BACKUP.SYNCHRONIZE_CLIPBOARD_HISTORY")}
            </h6>
          </div>

          <Toggle
            checked={!!SettingsStore.settings()?.sync}
            onChange={async () =>
              void (await SettingsStore.syncAuthenticateToggle())
            }
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
              max={MAX_SYNC_LIMIT}
              value={SettingsStore.settings()!.sync_limit}
              debounce={1000}
              onInput={async (e) => {
                const settings = await invokeCommand(
                  InvokeCommand.SyncLimitChange,
                  {
                    syncLimit: Number(e.target.value),
                  },
                );
                SettingsStore.setSettings(settings);
              }}
            />
          </div>
        </Show>
      </TextBlock>

      <TextBlock Icon={SiSqlite} title={t("SETTINGS.BACKUP.DATABASE_LOCATION")}>
        <div class="list-disc px-5 pt-2.5 pb-5">
          <div class="relative w-full cursor-pointer">
            <div
              title={databaseUrl()}
              class="dark:border-dark-light dark:bg-dark-light dark:focus:bg-dark-dark w-full truncate rounded-md border border-gray-300 px-3 py-0.5 text-left text-sm italic focus:outline-hidden dark:text-white"
            >
              {databaseUrl()}
            </div>
          </div>
          <div class="mt-1.5 flex items-center justify-end gap-1.5">
            <Button
              label="SETTINGS.BACKUP.RESET_LOCATION"
              onClick={async () => {
                await SettingsStore.resetClipboardDbLocation();
                setDatabaseUrl.refetch();
              }}
              Icon={BiRegularReset}
            />

            <Button
              label="SETTINGS.BACKUP.CHANGE_LOCATION"
              onClick={async () => {
                await SettingsStore.changeClipboardDbLocation();
                setDatabaseUrl.refetch();
              }}
              Icon={TbExchange}
            />

            <Button
              label="SETTINGS.BACKUP.OPEN"
              onClick={() =>
                invokeCommand(InvokeCommand.OpenFolder, {
                  location: FolderLocation.Database,
                })
              }
              Icon={AiTwotoneFolderOpen}
            />
          </div>
        </div>
      </TextBlock>

      <TextBlock
        Icon={BsGearWideConnected}
        title={t("SETTINGS.BACKUP.CONFIG_LOCATION")}
      >
        <div class="list-disc px-5 pt-2.5 pb-5">
          <div class="relative w-full cursor-pointer">
            <div
              title={configUrl()}
              class="dark:border-dark-light dark:bg-dark-light dark:focus:bg-dark-dark w-full truncate rounded-md border border-gray-300 px-3 py-0.5 text-left text-sm italic focus:outline-hidden dark:text-white"
            >
              {configUrl()}
            </div>
            <Button
              label="SETTINGS.BACKUP.OPEN"
              class="absolute inset-y-0 right-0"
              onClick={() =>
                invokeCommand(InvokeCommand.OpenFolder, {
                  location: FolderLocation.Config,
                })
              }
              Icon={AiTwotoneFolderOpen}
            />
          </div>
        </div>
      </TextBlock>
    </>
  );
};
