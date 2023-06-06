import { listen } from "@tauri-apps/api/event";
import { createEffect, createResource, onCleanup } from "solid-js";
import { render } from "solid-js/web";
import { Settings } from "./@types";
import App from "./App";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";
import { ExtendedHotKey } from "./utils/constants";

const Index = () => {
  const { initSettings, settings, updateSettings, updateHotkey } =
    SettingsStore;

  createResource(initSettings);

  createEffect(async () => {
    const refreshSettings = await listen<Settings>(
      "refresh_settings",
      ({ payload }) => {
        updateSettings(payload, false);
        initSettings();
      }
    );

    const refreshHotkeys = await listen<ExtendedHotKey>(
      "refresh_hotkeys",
      ({ payload }) => updateHotkey(payload, false)
    );

    onCleanup(() => {
      refreshSettings();
      refreshHotkeys();
    });
  });

  console.log("settings", settings());

  // if (!settings()) return null;

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
