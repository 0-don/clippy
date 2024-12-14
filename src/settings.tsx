import { createResource, onMount } from "solid-js";
import { Show, render } from "solid-js/web";
import { Tabs } from "./components/navigation/settings-tabs";
import { SettingsBackup } from "./components/pages/settings/settings-backup";
import { SettingsGeneral } from "./components/pages/settings/settings-general";
import { SettingsHistory } from "./components/pages/settings/settings-history";
import { SettingsHotkeys } from "./components/pages/settings/settings-hotkeys";
import AppStore from "./store/app-store";
import SettingsStore from "./store/settings-store";
import "./styles.css";
import { listenEvent } from "./utils/tauri";
import { ListenEvent } from "./types/tauri-listen";

const Settings = () => {
  const { getCurrentTab } = SettingsStore;
  const { init } = AppStore;

  createResource(init);

  onMount(() => listenEvent(ListenEvent.Init, init));

  return (
    <div class="absolute flex h-full w-full flex-col overflow-x-hidden bg-white text-black dark:bg-dark dark:text-white">
      <Tabs />
      <div class="px-5 pt-5 dark:text-white">
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
