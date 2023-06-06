import { createEffect } from "solid-js";
import { render } from "solid-js/web";
import App from "./App";
import AppStore from "./store/AppStore";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";

const Index = () => {
  const { initSettings, settings, updateSettings, updateHotkey } =
    SettingsStore;
  const { updateSidebarIcons } = AppStore;

  createEffect(() => {
    initSettings();

    // const refreshSettings = window.electron.on(
    //   "refreshSettings",
    //   (setting: Prisma.SettingsCreateInput) => {
    //     updateSettings(setting, false);
    //     initSettings();
    //   }
    // );

    // const refreshHotkeys = window.electron.on(
    //   "refreshHotkeys",
    //   (hotkey: ExtendedHotKey) => updateHotkey(hotkey, false)
    // );

    // return () => {
    //   refreshSettings();
    //   refreshHotkeys();
    // };
  });

  if (!settings) return null;

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
