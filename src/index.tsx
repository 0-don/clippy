import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { createEffect,createResource,onCleanup } from "solid-js";
import { render } from "solid-js/web";
import App from "./App";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";

const Index = () => {
  const { init } = SettingsStore;

  createResource(init);

  createEffect(async () => {
    const focus = await appWindow.onFocusChanged(
      async ({ payload: focused }) => !focused && (await appWindow.hide())
    );

    await invoke("init_listener");

    onCleanup(focus);
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
