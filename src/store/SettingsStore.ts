import { invoke } from "@tauri-apps/api";
import { IconTypes } from "solid-icons";
import { BsDatabaseFillGear } from "solid-icons/bs";
import { HiSolidCog8Tooth } from "solid-icons/hi";
import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { VsHistory } from "solid-icons/vs";
import { createRoot, createSignal } from "solid-js";
import { disable, enable } from "tauri-plugin-autostart-api";
import { Settings } from "../@types";
import HotkeyStore from "./HotkeyStore";

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
    setTabs((prev) =>
      prev.map((tab) => ({ ...tab, current: tab.name === tabName }))
    );

  const getCurrentTab = () => tabs().find((tab) => tab.current);

  const updateSettings = async (
    settings: Settings,
    upload: boolean | undefined = true
  ) => {
    if (upload) await invoke("update_settings", { settings });
    setSettings(settings);

    try {
      const env = import.meta.env;
      env.PROD && settings.startup ? await enable() : await disable();
    } catch (_) {}
  };

  const darkMode = () =>
    settings()?.dark_mode
      ? document.querySelector("html")?.classList?.add?.("dark")
      : document.querySelector("html")?.classList?.remove?.("dark");

  const init = async () => {
    await initSettings();
    HotkeyStore.initHotkeys(true);
    darkMode();
  };

  const initSettings = async () => {
    const settings = await invoke<Settings>("get_settings");
    setSettings(settings);

    try {
      const env = import.meta.env;
      env.PROD && settings.startup ? await enable() : await disable();
    } catch (_) {}
  };

  return {
    settings,
    setSettings,
    updateSettings,
    tabs,
    setTabs,
    setCurrentTab,
    getCurrentTab,
    init,
    initSettings,
    darkMode,
  };
}

export default createRoot(createSettingsStore);
