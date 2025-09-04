import { onMount } from "solid-js";
import { Show, render } from "solid-js/web";
import { SettingsBackup } from "./components/pages/settings/settings-backup";
import { SettingsEncryption } from "./components/pages/settings/settings-encryption";
import { SettingsGeneral } from "./components/pages/settings/settings-general";
import { SettingsHistory } from "./components/pages/settings/settings-history";
import { SettingsHotkeys } from "./components/pages/settings/settings-hotkeys";
import { SettingsLimits } from "./components/pages/settings/settings-limits";
import { SettingsPatterns } from "./components/pages/settings/settings-patterns";
import { Tabs } from "./components/pages/settings/settings-tabs";
import LanguageProvider from "./components/provider/language-provider";
import { listenEvent } from "./lib/tauri";
import { HotkeyStore } from "./store/hotkey-store";
import { SettingsStore } from "./store/settings-store";
import "./styles.css";
import { ListenEvent } from "./types/tauri-listen";

const Settings = () => {
  listenEvent(ListenEvent.InitSettings, SettingsStore.init);

  listenEvent(ListenEvent.InitHotkeys, HotkeyStore.init);

  onMount(() => {
    SettingsStore.init();
    HotkeyStore.init();
  });

  return (
    <div class="dark:bg-dark absolute flex h-full w-full flex-col overflow-x-hidden bg-white text-black dark:text-white">
      <Tabs />
      <div class="px-5 pt-5 dark:text-white">
        <Show
          when={SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.GENERAL"}
        >
          <SettingsGeneral />
        </Show>

        <Show
          when={SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.BACKUP"}
        >
          <SettingsBackup />
        </Show>

        <Show
          when={
            SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.ENCRYPTION"
          }
        >
          <SettingsEncryption />
        </Show>

        <Show
          when={SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.HISTORY"}
        >
          <SettingsHistory />
        </Show>

        <Show
          when={SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.HOTKEYS"}
        >
          <SettingsHotkeys />
        </Show>

        <Show
          when={SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.PATTERNS"}
        >
          <SettingsPatterns />
        </Show>

        <Show
          when={SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.LIMITS"}
        >
          <SettingsLimits />
        </Show>
      </div>
    </div>
  );
};

render(
  () => (
    <LanguageProvider>
      <Settings />
    </LanguageProvider>
  ),
  document.getElementById("root") as HTMLElement,
);
