import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { createResource, onMount } from "solid-js";
import { render } from "solid-js/web";
import App from "./components/pages/app/App";
import HotkeyStore from "./store/HotkeyStore";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";

const Index = () => {
  const { setGlobalHotkeyEvent } = HotkeyStore;
  const { init } = SettingsStore;

  createResource(init);

  onMount(async () => {
    setGlobalHotkeyEvent(true);

    const focus = await appWindow.onFocusChanged(
      async ({ payload }) =>
        !payload && (await invoke("window_display_toggle")),
    );

    const init_listener = await listen("init_listener", init);

    setTimeout(async () => {
      await invoke("stop_hotkeys");
      setGlobalHotkeyEvent(false);
    }, 5000);

    return async () => {
      init_listener();
      focus();
    };
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
