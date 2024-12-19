import { createResource, onMount } from "solid-js";
import { render } from "solid-js/web";
import App from "./components/pages/app/app";
import { AppStore } from "./store/app-store";
import { ClipboardStore } from "./store/clipboard-store";
import { HotkeyStore } from "./store/hotkey-store";
import "./styles.css";
import { ListenEvent } from "./types/tauri-listen";
import { listenEvent } from "./utils/tauri";

const Index = () => {
  createResource(AppStore.init);

  onMount(() => {
    listenEvent(ListenEvent.Init, AppStore.init);

    listenEvent(ListenEvent.EnableGlobalHotkeyEvent, HotkeyStore.setGlobalHotkeyEvent);

    listenEvent(ListenEvent.ChangeTab, AppStore.setCurrentTab);

    listenEvent(ListenEvent.NewClipboard, ClipboardStore.addClipboard);
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
