import { BsStarFill } from "solid-icons/bs";
import { CgMore } from "solid-icons/cg";
import { TbSearch } from "solid-icons/tb";
import { VsHistory } from "solid-icons/vs";
import { createResource, createRoot, createSignal } from "solid-js";
import { invokeCommand } from "../lib/tauri";
import { Tabs } from "../types";
import { HotkeyEvent } from "../types/enums";
import { InvokeCommand } from "../types/tauri-invoke";
import { LANGUAGES, LANGUAGE_KEY, TAB_NAMES, Tab } from "../utils/constants";
import { SettingsStore } from "./settings-store";

function createAppStore() {
  const detectedLocale = localStorage.getItem(LANGUAGE_KEY) || Object.values(LANGUAGES)[0];

  const [locale, setLocale] = createResource(
    async () => (await invokeCommand(InvokeCommand.GetSettings))?.language || detectedLocale
  );

  const [tabs, setTabs] = createSignal<Tabs[]>([
    {
      name: TAB_NAMES[0],
      Icon: VsHistory,
      current: true,
      id: HotkeyEvent.RecentClipboards,
    },
    {
      name: TAB_NAMES[1],
      Icon: BsStarFill,
      current: false,
      id: HotkeyEvent.StarredClipboards,
    },
    {
      name: TAB_NAMES[2],
      Icon: TbSearch,
      current: false,
      id: HotkeyEvent.History,
    },
    {
      name: TAB_NAMES[3],
      Icon: CgMore,
      current: false,
      id: HotkeyEvent.ViewMore,
    },
  ]);

  const changeTab = (id: Tab) => setTabs((prev) => prev.map((s) => ({ ...s, current: s.id === id })));
  const getCurrentTab = () => tabs().find((s) => s.current)!;

  const darkMode = () =>
    SettingsStore.settings()?.dark_mode
      ? document.querySelector("html")?.classList?.add?.("dark")
      : document.querySelector("html")?.classList?.remove?.("dark");

  return {
    locale,
    setLocale,
    tabs,
    setTabs,
    changeTab,
    getCurrentTab,
    darkMode,
  };
}

export const AppStore = createRoot(createAppStore);
export type AppStore = ReturnType<typeof createAppStore>;
