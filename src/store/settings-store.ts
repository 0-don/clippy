import { BsDatabaseFillGear } from "solid-icons/bs";
import { HiSolidCog8Tooth } from "solid-icons/hi";
import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { TbResize } from "solid-icons/tb";
import { VsHistory } from "solid-icons/vs";
import { createRoot, createSignal } from "solid-js";
import { invokeCommand } from "../lib/tauri";
import { Settings, SettingsTab } from "../types";
import { WebWindow } from "../types/enums";
import { InvokeCommand } from "../types/tauri-invoke";
import { SETTINGS_TAB, SettingsTabName } from "../utils/constants";

function createSettingsStore() {
  const [tabs, setTabs] = createSignal<SettingsTab[]>([
    { name: SETTINGS_TAB[0], Icon: HiSolidCog8Tooth, current: true },
    { name: SETTINGS_TAB[1], Icon: BsDatabaseFillGear, current: false },
    {
      name: SETTINGS_TAB[2],
      Icon: VsHistory,
      current: false,
    },
    {
      name: SETTINGS_TAB[3],
      Icon: RiDeviceKeyboardFill,
      current: false,
    },
    {
      name: SETTINGS_TAB[4],
      Icon: TbResize,
      current: false,
    },
  ]);
  const [settings, setSettings] = createSignal<Settings>();

  const setCurrentTab = (tabName: SettingsTabName) =>
    setTabs((prev) => prev.map((tab) => ({ ...tab, current: tab.name === tabName })));

  const getCurrentTab = () => tabs().find((tab) => tab.current);

  const updateSettings = async (settings: Settings, upload: boolean | undefined = true) => {
    if (upload) await invokeCommand(InvokeCommand.UpdateSettings, { settings });
    setSettings(settings);
    await invokeCommand(InvokeCommand.ToggleAutostart);
  };

  const initSettings = async () => {
    const settings = await invokeCommand(InvokeCommand.GetSettings);
    setSettings(settings);
  };

  const changeClipboardDbLocation = async () => invokeCommand(InvokeCommand.ChangeClipboardDbLocation);

  const syncAuthenticateToggle = async () => invokeCommand(InvokeCommand.SyncAuthenticateToggle);

  const openWindow = async (windowName: WebWindow, title: string) =>
    invokeCommand(InvokeCommand.OpenNewWindow, { windowName, title });

  const exitApp = async () => invokeCommand(InvokeCommand.ExitApp);

  return {
    settings,
    setSettings,
    updateSettings,
    tabs,
    setTabs,
    setCurrentTab,
    getCurrentTab,
    initSettings,
    changeClipboardDbLocation,
    syncAuthenticateToggle,
    openWindow,
    exitApp,
  };
}

export const SettingsStore = createRoot(createSettingsStore);
export type SettingsStore = ReturnType<typeof createSettingsStore>;
