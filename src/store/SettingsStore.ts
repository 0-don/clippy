import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
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

  const syncClipboard = async () => {
    let dir: string | undefined;

    const synchronize = !settings()?.synchronize;

    if (synchronize) {
      dir = (await open({
        directory: true,
        title: "Select Database Backup Location",
      })) as string;
    }

    if (dir === null) return;

    await updateSettings({
      ...settings()!,
      synchronize,
    });
    await invoke("sync_clipboard_history", { dir });
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
    syncClipboard,
  };
}

export default createRoot(createSettingsStore);
