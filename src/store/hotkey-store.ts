import { createRoot, createSignal } from "solid-js";
import { Hotkey } from "../types";
import { HotkeyEvent } from "../types/enums";
import { InvokeCommand } from "../types/tauri-invoke";
import { invokeCommand } from "../utils/tauri";

function createHotkeyStore() {
  const [globalHotkeyEvent, enableGlobalHotkeyEvent] = createSignal<boolean>(false);
  const [hotkeys, setHotkeys] = createSignal<Hotkey[]>([]);

  const updateHotkey = async (hotkey: Hotkey, upload: boolean | undefined = true) => {
    if (upload) await invokeCommand(InvokeCommand.UpdateHotkey, { hotkey });
    setHotkeys((prev) => prev.map((h) => (h.id === hotkey.id ? { ...h, ...hotkey } : h)));
  };

  const getHotkey = (event: HotkeyEvent) => hotkeys().find((h) => h.event === event);

  const initHotkeys = async () => {
    const hotkeys = await invokeCommand(InvokeCommand.GetHotkeys);
    setHotkeys(hotkeys);
  };

  return {
    globalHotkeyEvent,
    enableGlobalHotkeyEvent,
    hotkeys,
    setHotkeys,
    updateHotkey,
    initHotkeys,
    getHotkey,
  };
}

export const HotkeyStore = createRoot(createHotkeyStore);

export type HotkeyStore = ReturnType<typeof createHotkeyStore>;
