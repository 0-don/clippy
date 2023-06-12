import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { createEffect, createResource, onCleanup } from "solid-js";
import { render } from "solid-js/web";
import App from "./App";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";

const Index = () => {
  const { init } = SettingsStore;

  createResource(init);

  createEffect(() => {
    const focus = appWindow.onFocusChanged(
      async ({ payload: focused }) => !focused && (await appWindow.hide())
    );

    invoke("init_listener");
    init();

    onCleanup(async () => (await focus)());
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
