import { Component, onMount } from "solid-js";
import ClipboardStore from "../../../store/clipboard-store";
import { Clipboards } from "./clipboards";

interface RecentClipboardsProps {}

export const RecentClipboards: Component<RecentClipboardsProps> = ({}) => {
  const { setClipboards, getClipboards, resetWhere } = ClipboardStore;

  onMount(async () => {
    resetWhere();
    setClipboards(await getClipboards());
  });

  return <Clipboards />;
};
