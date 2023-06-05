import { AiFillStar, AiOutlineSearch } from "solid-icons/ai";
import { FaSolidEllipsis } from "solid-icons/fa";
import { VsHistory } from "solid-icons/vs";
import { createRoot, createSignal } from "solid-js";
import { Clipboards } from "../@types";
import { SidebarIcon, SidebarIconName } from "../utils/constants";

function createAppStore() {
  const [clipboards, setClipboards] = createSignal<Clipboards[]>([]);
  const [sidebarIcons, setSidebarIcons] = createSignal<SidebarIcon[]>([
    { name: "Recent Clipboards", Icon: VsHistory, current: true },
    { name: "Starred Clipboards", Icon: AiFillStar, current: false },
    {
      name: "History",
      Icon: AiOutlineSearch,
      current: false,
    },
    {
      name: "View more",
      Icon: FaSolidEllipsis,
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
