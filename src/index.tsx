import { listen } from "@tauri-apps/api/event";
import { unregisterAll } from "@tauri-apps/api/globalShortcut";
import { appWindow } from "@tauri-apps/api/window";
import { createEffect, createResource, onCleanup } from "solid-js";
import { render } from "solid-js/web";
import { Clips } from "./@types";
import App from "./App";
import AppStore from "./store/AppStore";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";

const Index = () => {
  const { setClipboards, clipboards } = AppStore;
  const { init } = SettingsStore;

  createEffect(() => {
    const focus = appWindow.onFocusChanged(
      async ({ payload: focused }) => !focused && (await appWindow.hide())
    );
    createResource(init);
    createResource(unregisterAll);
    const clipboardListener = listen<Clips>(
      "clipboard_listener",
      ({ payload }) => {
        setClipboards((prev) => [payload, ...prev]);
      }
    );

    console.log("listening for clipboard changes");

    onCleanup(async () => {
      console.log("cleaning up");
      (await clipboardListener)();
      (await focus)();
    });
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
