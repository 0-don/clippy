import { Component, onMount } from "solid-js";
import { ClipboardStore } from "../../../store/clipboard-store";
import { Clipboards } from "./clipboard/clipboards";

interface RecentClipboardsProps {}

export const RecentClipboards: Component<RecentClipboardsProps> = ({}) => {
  onMount(async () => {
    ClipboardStore.resetWhere();
    ClipboardStore.setClipboards(await ClipboardStore.getClipboards());
  });

  return <Clipboards />;
};
