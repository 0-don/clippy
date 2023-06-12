import { BsStarFill } from "solid-icons/bs";
import { CgMore } from "solid-icons/cg";
import { TbSearch } from "solid-icons/tb";
import { VsHistory } from "solid-icons/vs";
import { createRoot, createSignal } from "solid-js";
import { Clips } from "../@types";
import { SidebarIcon, SidebarIconName } from "../utils/constants";

function createAppStore() {
  const [clipboards, setClipboards] = createSignal<Clips[]>([]);
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
      prev.map((s) => ({ ...s, current: s.name === name }))
    );

  return {
    clipboards,
    setClipboards,
    sidebarIcons,
    setSidebarIcons,
    updateSidebarIcons,
  };
}

export default createRoot(createAppStore);
