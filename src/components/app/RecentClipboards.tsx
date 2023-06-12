import { listen } from "@tauri-apps/api/event";
import { Component, createEffect, onCleanup } from "solid-js";
import { Clips } from "../../@types";
import AppStore from "../../store/AppStore";
import { Clipboards } from "./Clipboards";

interface RecentClipboardsProps {}

export const RecentClipboards: Component<RecentClipboardsProps> = ({}) => {
  const { clipboards, setClipboards } = AppStore;

  createEffect(async () => {
    const addClipboard = await listen<Clips>("set_clipboard", ({ payload }) => {
      setClipboards([payload, ...clipboards()]);
    });

    onCleanup(addClipboard);
  });
  return <Clipboards />;
};
