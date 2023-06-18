import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { createEffect, createResource, onCleanup } from "solid-js";
import { render } from "solid-js/web";
import { Clips } from "./@types";
import App from "./App";
import AppStore from "./store/AppStore";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";

const Index = () => {
  const { setClipboards } = AppStore;
  const { init } = SettingsStore;

  createEffect(() => {
    console.log(1);
    const focus = appWindow.onFocusChanged(
      async ({ payload: focused }) => !focused && (await appWindow.hide())
    );
    createResource(init);

    const clipboardListener = listen<Clips>(
      "clipboard_listener",
      ({ payload }) => {
        setClipboards((prev) => [payload, ...prev]);
      }
    );

    onCleanup(async () => {
      (await clipboardListener)();
      (await focus)();
    });
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
