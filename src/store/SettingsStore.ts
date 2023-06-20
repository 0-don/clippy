import { invoke } from "@tauri-apps/api";
import { IconTypes } from "solid-icons";
import { BsDatabaseFillGear } from "solid-icons/bs";
import { HiSolidCog8Tooth } from "solid-icons/hi";
import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { VsHistory } from "solid-icons/vs";
import { createRoot, createSignal } from "solid-js";
import { enable } from "tauri-plugin-autostart-api";
import { Hotkey, HotkeyEvent, Settings } from "../@types";
import { parseShortcut, registerHotkeys } from "../utils/hotkeyRegister";

type SettingsTabName = "General" | "Backup" | "History" | "Hotkeys";

type SettingsTab = {
  name: SettingsTabName;
  Icon: IconTypes;
  current: boolean;
};

function createSettingsStore() {
  const [isProduction, setIsProduction] = createSignal<boolean>(false);
  const [globalHotkeyEvent, setGlobalHotkeyEvent] = createSignal<boolean>(true);
  const [hotkeys, setHotkeys] = createSignal<Hotkey[]>([]);
  const [settings, setSettings] = createSignal<Settings>();
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

  const setCurrentTab = (tabName: SettingsTabName) =>
    setTabs((prev) =>
      prev.map((tab) => ({ ...tab, current: tab.name === tabName }))
    );

  const getCurrentTab = () => tabs().find((tab) => tab.current);

  const updateSettings = async (
    settings: Settings,
    upload: boolean | undefined = true
  ) => {
    if (upload) await invoke("updateSettings", settings);
    setSettings(settings);
  };

  const updateHotkey = async (
    hotkey: Hotkey,
    upload: boolean | undefined = true
  ) => {
    // if (upload) await invoke("updateHotkey", hotkey as Hotkey);
    setHotkeys((prev) =>
      prev.map((h) => (h.id === hotkey.id ? { ...h, ...hotkey } : h))
    );
  };

  const getHotkey = (event: HotkeyEvent) =>
    hotkeys().find((h) => h.event === event);

  const updateIsProduction = async () => {
    const isProduction = await invoke<boolean>("is_production");
    setIsProduction(isProduction);
    if (isProduction) await enable();
  };

  const init = () => {
    updateIsProduction();
    initSettings();
    initHotkeys(true);
  };

  const initSettings = async () => {
    const settings = await invoke<Settings>("get_settings");
    setSettings(settings);
  };

  const initHotkeys = async (register: boolean = false) => {
    const hotkeys = (await invoke<Hotkey[]>("get_hotkeys")).map((h) => ({
      ...h,
      shortcut: parseShortcut(h),
    }));

    setHotkeys(hotkeys);

    if (register) {
      await registerHotkeys(hotkeys);
    }
  };

  return {
    globalHotkeyEvent,
    setGlobalHotkeyEvent,
    hotkeys,
    setHotkeys,
    settings,
    setSettings,
    tabs,
    setTabs,
    setCurrentTab,
    updateSettings,
    updateHotkey,
    init,
    isProduction,
    getCurrentTab,
    initSettings,
    initHotkeys,
    getHotkey,
  };
}

export default createRoot(createSettingsStore);
