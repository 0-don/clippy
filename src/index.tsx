import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { createEffect, createResource, onCleanup } from "solid-js";
import { render } from "solid-js/web";
import { Clips } from "./@types";
import App from "./components/pages/app/App";
import ClipboardStore from "./store/ClipboardStore";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";

const Index = () => {
  const { setClipboards } = ClipboardStore;
  const { init } = SettingsStore;

  createResource(init);

  createEffect(() => {
    const focus = appWindow.onFocusChanged(
      async ({ payload }) => !payload && (await appWindow.hide())
    );

    const clipboardListener = listen<Clips>(
      "clipboard_listener",
      ({ payload }) => setClipboards((prev) => [payload, ...prev])
    );

    const initLisiner = listen("init_listener", init);

    onCleanup(async () => {
      (await clipboardListener)();
      (await focus)();
      (await initLisiner)();
    });
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
