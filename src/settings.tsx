import { listen } from "@tauri-apps/api/event";
import { createResource, onMount } from "solid-js";
import { Show, render } from "solid-js/web";
import { Tabs } from "./components/navigation/SettingsTabs";
import { SettingsBackup } from "./components/pages/settings/SettingsBackup";
import { SettingsGeneral } from "./components/pages/settings/SettingsGeneral";
import { SettingsHistory } from "./components/pages/settings/SettingsHistory";
import { SettingsHotkeys } from "./components/pages/settings/SettingsHotkeys";
import AppStore from "./store/AppStore";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";

const Settings = () => {
  const { getCurrentTab } = SettingsStore;
  const { init } = AppStore;

  createResource(init);

  onMount(() => listen("init", init));

  return (
    <div class="absolute flex h-full w-full flex-col bg-white text-black dark:bg-dark dark:text-white">
      <Tabs />
      <div class="p-5 dark:text-white">
        <Show when={getCurrentTab()?.name === "General"}>
          <SettingsGeneral />
        </Show>

        <Show when={getCurrentTab()?.name === "Backup"}>
          <SettingsBackup />
        </Show>

        <Show when={getCurrentTab()?.name === "History"}>
          <SettingsHistory />
        </Show>

        <Show when={getCurrentTab()?.name === "Hotkeys"}>
          <SettingsHotkeys />
        </Show>
      </div>
    </div>
  );
};

render(() => <Settings />, document.getElementById("root") as HTMLElement);
