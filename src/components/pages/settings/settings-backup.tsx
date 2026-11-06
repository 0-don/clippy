import { AiTwotoneFolderOpen } from "solid-icons/ai";
import { BiRegularReset } from "solid-icons/bi";
import { BsGearWideConnected } from "solid-icons/bs";
import { SiSqlite } from "solid-icons/si";
import {
  TbOutlineCloud,
  TbOutlineDatabaseStar,
  TbOutlineDevices,
  TbOutlineExchange,
  TbOutlineMail,
  TbOutlineNumbers,
  TbOutlinePlugConnectedX,
  TbOutlineRefresh,
} from "solid-icons/tb";
import {
  Component,
  createResource,
  createSignal,
  For,
  onCleanup,
  onMount,
  Show,
} from "solid-js";
import { invokeCommand, listenEvent } from "../../../lib/tauri";
import { HotkeyStore } from "../../../store/hotkey-store";
import { SettingsStore } from "../../../store/settings-store";
import { FolderLocation, HotkeyEvent } from "../../../types/enums";
import { Devices, SyncStatus } from "../../../types";
import { InvokeCommand } from "../../../types/tauri-invoke";
import { ListenEvent } from "../../../types/tauri-listen";
import { MAX_SYNC_LIMIT } from "../../../utils/constants";
import dayjs from "../../../utils/dayjs";
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
  const [syncStatus, setSyncStatus] = createSignal<SyncStatus>();
  const [devices, setDevices] = createSignal<Devices>();
  const loadDevices = async () =>
    setDevices(await invokeCommand(InvokeCommand.GetDevices));
  const [now, setNow] = createSignal(Date.now());

  onMount(async () => {
    setSyncStatus(await invokeCommand(InvokeCommand.SyncStatus));
    await loadDevices();
  });
  const unlisten = listenEvent(ListenEvent.SyncStatus, (status) => {
    setSyncStatus(status);
    void loadDevices();
  });
  const clock = setInterval(() => setNow(Date.now()), 30_000);
  onCleanup(() => {
    clearInterval(clock);
    void unlisten.then((stop) => stop());
  });

  const fromNow = (unixSeconds: number) => {
    now();
    return dayjs
      .unix(unixSeconds)
      .locale(SettingsStore.settings()?.language || "en")
      .fromNow();
  };

  const statusText = () => {
    const status = syncStatus();
    switch (status?.state) {
      case "synced":
        return t("SETTINGS.BACKUP.STATUS_SYNCED", {
          time: fromNow(status.last_synced_at ?? 0),
        });
      case "auth_error":
        return t("SETTINGS.BACKUP.STATUS_AUTH_ERROR");
      case "error":
        return t("SETTINGS.BACKUP.STATUS_ERROR");
      default:
        return t("SETTINGS.BACKUP.STATUS_CONNECTING");
    }
  };

  const hasProblem = () =>
    syncStatus()?.state === "auth_error" || syncStatus()?.state === "error";

  return (
    <>
      <TextBlock Icon={TbOutlineDatabaseStar} title={t("SETTINGS.BACKUP.SYNC")}>
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
              <TbOutlineMail />
              <h6 class="text-sm">{t("SETTINGS.BACKUP.ACCOUNT")}</h6>
            </div>
            <span class="truncate text-sm text-muted-foreground">
              {syncStatus()?.email ?? "..."}
            </span>
          </div>

          <div class="flex items-center justify-between space-x-2 px-5 pb-5">
            <div class="flex flex-col truncate">
              <div class="flex items-center space-x-2">
                <TbOutlineCloud />
                <h6
                  class="text-sm"
                  classList={{ "text-red-500": hasProblem() }}
                >
                  {statusText()}
                </h6>
              </div>
              <Show when={syncStatus()?.state === "error" && syncStatus()?.error}>
                <p
                  class="ml-6 truncate text-xs text-muted-foreground"
                  title={syncStatus()?.error ?? ""}
                >
                  {syncStatus()?.error}
                </p>
              </Show>
            </div>
            <div class="flex shrink-0 gap-2">
              <Show when={hasProblem()}>
                <Button
                  label="SETTINGS.BACKUP.RECONNECT"
                  onClick={() => invokeCommand(InvokeCommand.SyncReconnect)}
                  Icon={TbOutlineRefresh}
                />
              </Show>
              <Button
                label="SETTINGS.BACKUP.DISCONNECT"
                onClick={() => invokeCommand(InvokeCommand.SyncDisconnect)}
                Icon={TbOutlinePlugConnectedX}
              />
            </div>
          </div>

          <Show when={devices()?.devices.length}>
            <div class="flex flex-col gap-1.5 px-5 pb-5">
              <div class="flex items-center space-x-2">
                <TbOutlineDevices />
                <h6 class="text-sm">{t("SETTINGS.BACKUP.DEVICES")}</h6>
              </div>
              <For each={devices()!.devices}>
                {(device) => (
                  <div class="ml-6 flex items-center justify-between gap-2 text-xs">
                    <span class="truncate">
                      {device.name}{" "}
                      <span class="text-muted-foreground">({device.os})</span>
                    </span>
                    <span class="shrink-0 text-muted-foreground">
                      {device.id === devices()!.this_device
                        ? t("SETTINGS.BACKUP.THIS_DEVICE")
                        : t("SETTINGS.BACKUP.LAST_SEEN", {
                            time: fromNow(device.last_seen),
                          })}
                    </span>
                  </div>
                )}
              </For>
            </div>
          </Show>

          <div class="flex items-center justify-between space-x-2 px-5 pb-5">
            <div class="flex items-center space-x-2 truncate">
              <TbOutlineNumbers />
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
              class="w-full truncate rounded-md border border-border bg-popover px-3 py-0.5 text-left text-sm italic focus:bg-muted focus:outline-hidden text-foreground"
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
              Icon={TbOutlineExchange}
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
              class="w-full truncate rounded-md border border-border bg-popover px-3 py-0.5 text-left text-sm italic focus:bg-muted focus:outline-hidden text-foreground"
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
