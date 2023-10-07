import { listen } from "@tauri-apps/api/event";
import { createResource, onMount } from "solid-js";
import { render } from "solid-js/web";
import App from "./components/pages/app/App";
import HotkeyStore from "./store/HotkeyStore";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";

const Index = () => {
  let timer: NodeJS.Timeout;
  const { setGlobalHotkeyEvent } = HotkeyStore;
  const { init } = SettingsStore;

  createResource(init);

  onMount(async () => {
    const globalHotkeyListen = await listen(
      "set_global_hotkey_event",
      ({ payload }) => setGlobalHotkeyEvent(!!payload),
    );

    const initListen = await listen("init", init);

    return () => {
      globalHotkeyListen();
      initListen();
    };
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
