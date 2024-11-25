import { invoke } from "@tauri-apps/api/core";
import { createRoot, createSignal } from "solid-js";
import { Hotkey, HotkeyEvent } from "../@types";

function createHotkeyStore() {
  const [globalHotkeyEvent, setGlobalHotkeyEvent] = createSignal<boolean>(false);
  const [hotkeys, setHotkeys] = createSignal<Hotkey[]>([]);

  const updateHotkey = async (hotkey: Hotkey, upload: boolean | undefined = true) => {
    if (upload) await invoke("update_hotkey", { hotkey });
    setHotkeys((prev) => prev.map((h) => (h.id === hotkey.id ? { ...h, ...hotkey } : h)));
  };

  const getHotkey = (event: HotkeyEvent) => hotkeys().find((h) => h.event === event);

  const initHotkeys = async () => {
    const hotkeys = await invoke<Hotkey[]>("get_hotkeys");
    setHotkeys(hotkeys);
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
