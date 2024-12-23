import { createResource, onMount } from "solid-js";
import { Show, render } from "solid-js/web";
import { SettingsBackup } from "./components/pages/settings/settings-backup";
import { SettingsGeneral } from "./components/pages/settings/settings-general";
import { SettingsHistory } from "./components/pages/settings/settings-history";
import { SettingsHotkeys } from "./components/pages/settings/settings-hotkeys";
import { SettingsLimits } from "./components/pages/settings/settings-limits";
import { Tabs } from "./components/pages/settings/settings-tabs";
import LanguageProvider from "./components/provider/language-provider";
import { listenEvent } from "./lib/tauri";
import { AppStore } from "./store/app-store";
import { SettingsStore } from "./store/settings-store";
import "./styles.css";
import { ListenEvent } from "./types/tauri-listen";

const Settings = () => {
  createResource(AppStore.init);

  onMount(() => listenEvent(ListenEvent.Init, AppStore.init));

  return (
    <div class="absolute flex h-full w-full flex-col overflow-x-hidden bg-white text-black dark:bg-dark dark:text-white">
      <Tabs />
      <div class="px-5 pt-5 dark:text-white">
        <Show when={SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.GENERAL"}>
          <SettingsGeneral />
        </Show>

        <Show when={SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.BACKUP"}>
          <SettingsBackup />
        </Show>

        <Show when={SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.HISTORY"}>
          <SettingsHistory />
        </Show>

        <Show when={SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.HOTKEYS"}>
          <SettingsHotkeys />
        </Show>

        <Show when={SettingsStore.getCurrentTab()?.name === "SETTINGS.TAB.LIMITS"}>
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
  document.getElementById("root") as HTMLElement
);
