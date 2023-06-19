import { Component, createEffect } from "solid-js";
import ClipboardStore from "../../store/ClipboardStore";
import { Clipboards } from "./Clipboards";

interface RecentClipboardsProps {}

export const RecentClipboards: Component<RecentClipboardsProps> = ({}) => {
  const { setClipboards, getClipboards, resetWhere } = ClipboardStore;

  createEffect(async () => {
    resetWhere();
    setClipboards(await getClipboards());
  });

  return <Clipboards />;
};
