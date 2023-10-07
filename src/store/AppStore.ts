import { BsStarFill } from "solid-icons/bs";
import { CgMore } from "solid-icons/cg";
import { TbSearch } from "solid-icons/tb";
import { VsHistory } from "solid-icons/vs";
import { createRoot, createSignal } from "solid-js";
import { SidebarIcon, SidebarIconName } from "../utils/constants";
import ClipboardStore from "./ClipboardStore";
import HotkeyStore from "./HotkeyStore";
import SettingsStore from "./SettingsStore";
function createAppStore() {
  const [sidebarIcons, setSidebarIcons] = createSignal<SidebarIcon[]>([
    { name: "Recent Clipboards", Icon: VsHistory, current: true },
    { name: "Starred Clipboards", Icon: BsStarFill, current: false },
    {
      name: "History",
      Icon: TbSearch,
      current: false,
    },
    {
      name: "View more",
      Icon: CgMore,
      current: false,
    },
  ]);

  const updateSidebarIcons = (name: SidebarIconName) =>
    setSidebarIcons((prev) =>
      prev.map((s) => ({ ...s, current: s.name === name })),
    );

  const getCurrentSidebarIcon = () => sidebarIcons().find((s) => s.current);

  const sIcon = () => sidebarIcons().find((s) => s.current);

  const init = async () => {
    SettingsStore.initSettings();
    HotkeyStore.initHotkeys();
    ClipboardStore.initClipboards();
    darkMode();
  };

  const darkMode = () =>
    SettingsStore.settings()?.dark_mode
      ? document.querySelector("html")?.classList?.add?.("dark")
      : document.querySelector("html")?.classList?.remove?.("dark");

  return {
    sidebarIcons,
    setSidebarIcons,
    updateSidebarIcons,
    getCurrentSidebarIcon,
    sIcon,
    init,
    darkMode,
  };
}

export default createRoot(createAppStore);
