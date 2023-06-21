import { invoke } from "@tauri-apps/api";
import { createRoot, createSignal } from "solid-js";
import { Hotkey, HotkeyEvent } from "../@types";
import { parseShortcut, registerHotkeys } from "../utils/hotkeyRegister";

function createHotkeyStore() {
  const [globalHotkeyEvent, setGlobalHotkeyEvent] = createSignal<boolean>(true);
  const [hotkeys, setHotkeys] = createSignal<Hotkey[]>([]);

  const updateHotkey = async (
    hotkey: Hotkey,
    upload: boolean | undefined = true
  ) => {
    if (upload) await invoke("update_hotkey", { hotkey });
    setHotkeys((prev) =>
      prev.map((h) => (h.id === hotkey.id ? { ...h, ...hotkey } : h))
    );
  };

  const getHotkey = (event: HotkeyEvent) =>
    hotkeys().find((h) => h.event === event);

  const initHotkeys = async (register: boolean | undefined = false) => {
    const hotkeys = (await invoke<Hotkey[]>("get_hotkeys")).map((h) => ({
      ...h,
      shortcut: parseShortcut(h),
    }));

    setHotkeys(hotkeys);

    
    if (register) await registerHotkeys(hotkeys);
  };

  return {
    globalHotkeyEvent,
    setGlobalHotkeyEvent,
    hotkeys,
    setHotkeys,
    updateHotkey,
    initHotkeys,
    getHotkey,
  };
}

export default createRoot(createHotkeyStore);
