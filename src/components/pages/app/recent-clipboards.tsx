import { Component, onSettled } from "solid-js";
import { ClipboardStore, initialWhere } from "../../../store/clipboard-store";
import { Clipboards } from "./clipboard/clipboards";

export const RecentClipboards: Component = () => {
  onSettled(() => {
    void ClipboardStore.getClipboards(initialWhere).then(
      ClipboardStore.setClipboards,
    );
  });

  return <Clipboards />;
};
