import { Component } from "solid-js";
import { ClipboardStore } from "../../../store/clipboard-store";
import { Clipboards } from "./clipboard/clipboards";

interface RecentClipboardsProps {}

export const RecentClipboards: Component<RecentClipboardsProps> = ({}) => {
  const { setClipboards, getClipboards, resetWhere } = ClipboardStore;

  return <Clipboards />;
};
