import { appWindow } from "@tauri-apps/api/window";
import { createEffect, createResource, onCleanup } from "solid-js";
import { render } from "solid-js/web";
import App from "./App";
import { ClipboardListener } from "./components/utils/ClipboardListener";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";

const Index = () => {
  const { initSettings } = SettingsStore;

  createResource(initSettings);

  ClipboardListener();

  createEffect(async () => {
    const focus = await appWindow.onFocusChanged(
      async ({ payload: focused }) => !focused && (await appWindow.hide())
    );

    onCleanup(focus);
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
