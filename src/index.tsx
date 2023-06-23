import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { createResource, onMount } from "solid-js";
import { render } from "solid-js/web";
import { Clips } from "./@types";
import App from "./components/pages/app/App";
import ClipboardStore from "./store/ClipboardStore";
import HotkeyStore from "./store/HotkeyStore";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";
import { removeAllHotkeyListeners } from "./utils/hotkeyRegister";

const Index = () => {
  const { initHotkeys } = HotkeyStore;
  const { setClipboards } = ClipboardStore;
  const { init } = SettingsStore;

  createResource(init);

  onMount(async () => {
    const focus = appWindow.onFocusChanged(async ({ payload }) => {
      if (!payload) {
        await appWindow.hide();
        removeAllHotkeyListeners();
      }
    });

    const clipboardListener = listen<Clips>(
      "clipboard_listener",
      ({ payload }) => setClipboards((prev) => [payload, ...prev])
    );

    const initLisiner = listen("init_listener", init);

    const initHotkeysListener = listen("init_hotkeys_listener", () =>
      initHotkeys(true)
    );

    // onCleanup(async () => {
    //   (await clipboardListener)();
    //   (await focus)();
    //   (await initLisiner)();
    // });
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
