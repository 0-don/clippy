import { BsStarFill } from "solid-icons/bs";
import { CgMore } from "solid-icons/cg";
import { VsHistory } from "solid-icons/vs";
import { createRoot, createSignal } from "solid-js";
import { TabId, Tabs } from "../utils/constants";
import ClipboardStore from "./ClipboardStore";
import HotkeyStore from "./HotkeyStore";
import SettingsStore from "./SettingsStore";

function createAppStore() {
  const [tabs, setTabs] = createSignal<Tabs[]>([
    {
      name: "Recent Clipboards",
      Icon: VsHistory,
      current: true,
      id: "recent_clipboards",
    },
    {
      name: "Starred Clipboards",
      Icon: BsStarFill,
      current: false,
      id: "starred_clipboards",
    },
    {
      name: "View more",
      Icon: CgMore,
      current: false,
      id: "view_more",
    },
  ]);

  const setCurrentTab = (id: TabId) => setTabs((prev) => prev.map((s) => ({ ...s, current: s.id === id })));

  const getCurrentTab = () => tabs().find((s) => s.current);

  const tIcon = () => tabs().find((s) => s.current);

  const init = async () => {
    HotkeyStore.initHotkeys();
    ClipboardStore.initClipboards();
    await SettingsStore.initSettings();
    darkMode();
  };

  const darkMode = () =>
    SettingsStore.settings()?.dark_mode
      ? document.querySelector("html")?.classList?.add?.("dark")
      : document.querySelector("html")?.classList?.remove?.("dark");

  return {
    tabs,
    setTabs,
    setCurrentTab,
    getCurrentTab,
    tIcon,
    init,
    darkMode,
  };
}

export default createRoot(createAppStore);
