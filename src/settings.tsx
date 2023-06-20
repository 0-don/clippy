import { Show, render } from "solid-js/web";

import { SettingsAccount } from "./components/pages/settings/SettingsAcount";
import { SettingsGeneral } from "./components/pages/settings/SettingsGeneral";
import { SettingsHistory } from "./components/pages/settings/SettingsHistory";
import { SettingsHotkeys } from "./components/pages/settings/SettingsHotkeys";
import { Tabs } from "./components/utils/Tabs";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";

const Settings = () => {
  const { tabs, getCurrentTab } = SettingsStore;

  return (
    <div class="absolute flex h-full w-full flex-col overflow-hidden bg-white text-black dark:bg-dark dark:text-white">
      <Tabs />
      <div class="p-5 dark:text-white">
        <Show when={getCurrentTab()?.name === "General"}>
          <SettingsGeneral />
        </Show>

        <Show when={getCurrentTab()?.name === "Account"}>
          <SettingsAccount />
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
