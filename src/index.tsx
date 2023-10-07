import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
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

  onMount(() => {
    // window.onfocus = async () => {
    //   setGlobalHotkeyEvent(true);
    //   clearInterval(timer);
    //   timer = setTimeout(async () => {
    //     setGlobalHotkeyEvent(false);
    //     invoke("stop_hotkeys");
    //   }, 5000);
    // };

    // window.onblur = async () => {
    //   clearInterval(timer);
    //   invoke("stop_hotkeys");
    //   setGlobalHotkeyEvent(false);
    //   appWindow.hide();
    // };
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
