import { Component, onMount } from "solid-js";
import { Clipboards } from "./clipboard/clipboards";
import { ClipboardStore } from "../../../store/clipboard-store";

interface RecentClipboardsProps {}

export const RecentClipboards: Component<RecentClipboardsProps> = ({}) => {
  const { setClipboards, getClipboards, resetWhere } = ClipboardStore;

  onMount(async () => {
    resetWhere();
    setClipboards(await getClipboards());
  });

  return <Clipboards />;
};
