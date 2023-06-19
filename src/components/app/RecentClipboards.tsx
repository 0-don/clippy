import { Component, createEffect } from "solid-js";
import ClipboardStore from "../../store/ClipboardStore";
import { Clipboards } from "./Clipboards";

interface RecentClipboardsProps {}

export const RecentClipboards: Component<RecentClipboardsProps> = ({}) => {
  const { setClipboards, getClipboards } = ClipboardStore;
  createEffect(async () => setClipboards(await getClipboards()));
  return <Clipboards />;
};
