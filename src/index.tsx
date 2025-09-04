import { onMount } from "solid-js";
import { render } from "solid-js/web";
import App from "./components/pages/app/app";
import LanguageProvider from "./components/provider/language-provider";
import { listenEvent } from "./lib/tauri";
import { AppStore } from "./store/app-store";
import { ClipboardStore } from "./store/clipboard-store";
import { HotkeyStore } from "./store/hotkey-store";
import { SettingsStore } from "./store/settings-store";
import "./styles.css";
import { ListenEvent } from "./types/tauri-listen";

const Index = () => {
  onMount(() => {
    SettingsStore.init();
    HotkeyStore.init();
  });

  listenEvent(ListenEvent.InitClipboards, ClipboardStore.init);

  listenEvent(ListenEvent.InitSettings, SettingsStore.init);

  listenEvent(ListenEvent.InitHotkeys, HotkeyStore.init);

  listenEvent(
    ListenEvent.EnableGlobalHotkeyEvent,
    HotkeyStore.enableGlobalHotkeyEvent,
  );

  listenEvent(ListenEvent.ChangeTab, AppStore.changeTab);

  listenEvent(ListenEvent.NewClipboard, ClipboardStore.newClipboard);

  listenEvent(ListenEvent.PasswordLock, AppStore.setPasswordLock);

  return <App />;
};

render(
  () => (
    <LanguageProvider>
      <Index />
    </LanguageProvider>
  ),
  document.getElementById("root") as HTMLElement,
);
