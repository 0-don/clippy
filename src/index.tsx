import { listen } from "@tauri-apps/api/event";
import { createResource, onMount } from "solid-js";
import { render } from "solid-js/web";
import { WindowName } from "./@types";
import App from "./components/pages/app/App";
import AppStore from "./store/AppStore";
import HotkeyStore from "./store/HotkeyStore";
import "./styles.css";
import { TabId } from "./utils/constants";
import { openWindow } from "./utils/helpers";

const Index = () => {
  const { setGlobalHotkeyEvent } = HotkeyStore;
  const { init, setCurrentTab } = AppStore;

  createResource(init);

  onMount(() => {
    listen("set_global_hotkey_event", ({ payload }) => setGlobalHotkeyEvent(!!payload));

    listen("init", init);

    listen("open_window", ({ payload }: { payload: WindowName }) => openWindow(payload));

    listen("change_tab", ({ payload }: { payload: TabId }) => setCurrentTab(payload));
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
