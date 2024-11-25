import { invoke } from "@tauri-apps/api/core";
import { IconTypes } from "solid-icons";
import { BsDatabaseFillGear } from "solid-icons/bs";
import { HiSolidCog8Tooth } from "solid-icons/hi";
import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { VsHistory } from "solid-icons/vs";
import { createRoot, createSignal } from "solid-js";
import { Settings, WindowName } from "../@types";

type SettingsTabName = "General" | "Backup" | "History" | "Hotkeys";

type SettingsTab = {
  name: SettingsTabName;
  Icon: IconTypes;
  current: boolean;
};

function createSettingsStore() {
  const [tabs, setTabs] = createSignal<SettingsTab[]>([
    { name: "General", Icon: HiSolidCog8Tooth, current: true },
    { name: "Backup", Icon: BsDatabaseFillGear, current: false },
    {
      name: "History",
      Icon: VsHistory,
      current: false,
    },
    {
      name: "Hotkeys",
      Icon: RiDeviceKeyboardFill,
      current: false,
    },
  ]);
  const [settings, setSettings] = createSignal<Settings>();

  const setCurrentTab = (tabName: SettingsTabName) =>
    setTabs((prev) => prev.map((tab) => ({ ...tab, current: tab.name === tabName })));

  const getCurrentTab = () => tabs().find((tab) => tab.current);

  const updateSettings = async (settings: Settings, upload: boolean | undefined = true) => {
    if (upload) await invoke("update_settings", { settings });
    setSettings(settings);
    await invoke("toggle_autostart");
  };

  const initSettings = async () => {
    const settings = await invoke<Settings>("get_settings");
    setSettings(settings);
  };

  const syncClipboard = async () => invoke<void>("sync_clipboard_history") 

  const openWindow = async (windowName: WindowName) => invoke("open_new_window", { windowName });

  const exitApp = async () => invoke("exit_app");

  return {
    settings,
    setSettings,
    updateSettings,
    tabs,
    setTabs,
    setCurrentTab,
    getCurrentTab,
    initSettings,
    syncClipboard,
    openWindow,
    exitApp,
  };
}

export default createRoot(createSettingsStore);
