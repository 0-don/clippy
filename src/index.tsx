import { createResource, onMount } from "solid-js";
import { render } from "solid-js/web";
import App from "./components/pages/app/app";
import { listenEvent } from "./lib/tauri";
import { AppStore } from "./store/app-store";
import { ClipboardStore } from "./store/clipboard-store";
import { HotkeyStore } from "./store/hotkey-store";
import "./styles.css";
import { ListenEvent } from "./types/tauri-listen";
import LanguageProvider from "./components/provider/language-provider";

const Index = () => {
  createResource(AppStore.init);

  onMount(() => {
    listenEvent(ListenEvent.Init, AppStore.init);

    listenEvent(ListenEvent.EnableGlobalHotkeyEvent, HotkeyStore.enableGlobalHotkeyEvent);

    listenEvent(ListenEvent.ChangeTab, AppStore.changeTab);

    listenEvent(ListenEvent.NewClipboard, ClipboardStore.newClipboard);
  });

  return <App />;
};

render(
  () => (
    <LanguageProvider>
      <Index />
    </LanguageProvider>
  ),
  document.getElementById("root") as HTMLElement
);
