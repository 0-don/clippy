import { listen } from "@tauri-apps/api/event";
import { sendNotification } from "@tauri-apps/api/notification";
import { appWindow } from "@tauri-apps/api/window";
import { createResource, onMount } from "solid-js";
import { render } from "solid-js/web";
import { Clips } from "./@types";
import App from "./components/pages/app/App";
import ClipboardStore from "./store/ClipboardStore";
import HotkeyStore from "./store/HotkeyStore";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";
import { registerHotkeys } from "./utils/hotkeyRegister";

const Index = () => {
  const { hotkeys } = HotkeyStore;
  const { setClipboards } = ClipboardStore;
  const { init, settings } = SettingsStore;

  createResource(init);

  onMount(async () => {
    appWindow.onFocusChanged(({ payload }) => {
      console.log(payload);
      // if (!payload) {
      //   appWindow.hide();
      //   removeAllHotkeyListeners();
      // }
    });

    const clipboard_listener = await listen<Clips>(
      "clipboard_listener",
      ({ payload }) => {
        settings()?.notification &&
          sendNotification({
            title: `New ${payload.type}`,
            body: "Copied to clipboard",
          });
        setClipboards((prev) => [payload, ...prev]);
      },
    );

    const init_listener = await listen("init_listener", init);

    const init_hotkeys_listener = await listen("init_hotkeys_listener", () => {
      registerHotkeys(hotkeys());
    });

    return async () => {
      clipboard_listener();
      init_listener();
      init_hotkeys_listener();
    };
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
