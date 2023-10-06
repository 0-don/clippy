import { listen } from "@tauri-apps/api/event";
import { createResource, onMount } from "solid-js";
import { render } from "solid-js/web";
import App from "./components/pages/app/App";
import HotkeyStore from "./store/HotkeyStore";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";
import { invoke } from "@tauri-apps/api";

const Index = () => {
  const { setGlobalHotkeyEvent } = HotkeyStore;
  const { init } = SettingsStore;

  createResource(init);

  onMount(async () => {
    window.onfocus = async () => {
      setGlobalHotkeyEvent(true);

      setTimeout(async () => {
        setGlobalHotkeyEvent(false);
      }, 5000);
    };

    window.onblur = async () => {
      await invoke("window_display_toggle");
      setGlobalHotkeyEvent(false);
    };

    // const focus = await appWindow.onFocusChanged(
    //   async ({ payload }) =>
    //     !payload && (await invoke("window_display_toggle")),
    // );

    const init_listener = await listen("init_listener", init);

    return async () => {
      init_listener();
      focus();
    };
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
