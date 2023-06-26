import { Component, createEffect, onMount } from "solid-js";
import ClipboardStore from "../../../store/ClipboardStore";
import { Clipboards } from "./Clipboards";

interface RecentClipboardsProps {}

export const RecentClipboards: Component<RecentClipboardsProps> = ({}) => {
  const { setClipboards, getClipboards, resetWhere } = ClipboardStore;

  onMount(async () => {
    resetWhere();
    setClipboards(await getClipboards());
  });

  return <Clipboards />;
};
