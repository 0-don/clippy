import { createResource, onMount } from "solid-js";
import { render } from "solid-js/web";
import App from "./components/pages/app/app";
import "./styles.css";
import { ListenEvent } from "./types/tauri-listen";
import { listenEvent } from "./utils/tauri";
import { AppStore } from "./store/app-store";
import { HotkeyStore } from "./store/hotkey-store";

const Index = () => {
  const { setGlobalHotkeyEvent } = HotkeyStore;
  const { init, setCurrentTab } = AppStore;

  createResource(init);

  onMount(() => {
    listenEvent(ListenEvent.Init, init);

    listenEvent(ListenEvent.SetGlobalHotkeyEvent, (bool) => setGlobalHotkeyEvent(bool));

    listenEvent(ListenEvent.ChangeTab, (tab) => setCurrentTab(tab));
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
