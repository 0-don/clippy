import { Component, createEffect } from "solid-js";
import ClipboardStore, { initialWhere } from "../../../store/ClipboardStore";
import { Clipboards } from "./Clipboards";

interface StarredClipboardsProps {}

export const StarredClipboards: Component<StarredClipboardsProps> = ({}) => {
  const { setClipboards, getClipboards, setWhere } = ClipboardStore;

  createEffect(async () => {
    setWhere(() => ({ ...initialWhere, star: true }));
    setClipboards(await getClipboards());
  });

  return <Clipboards />;
};
