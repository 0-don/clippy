import { listen } from "@tauri-apps/api/event";
import { createResource, onMount } from "solid-js";
import { render } from "solid-js/web";
import App from "./components/pages/app/App";
import AppStore from "./store/AppStore";
import HotkeyStore from "./store/HotkeyStore";
import "./styles.css";

const Index = () => {
  const { setGlobalHotkeyEvent } = HotkeyStore;
  const { init } = AppStore;

  createResource(init);

  onMount(() => {
    listen("set_global_hotkey_event", ({ payload }) =>
      setGlobalHotkeyEvent(!!payload),
    );

    listen("init", init);
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
