import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { register } from "@tauri-apps/api/globalShortcut";
import { BsHddFill } from "solid-icons/bs";
import { FiGlobe } from "solid-icons/fi";
import { createEffect,createResource } from "solid-js";
import { Hotkey } from "./@types";
import { History } from "./components/app/History";
import { RecentClipboards } from "./components/app/RecentClipboards";
import { StarredClipboards } from "./components/app/StarredClipboards";
import { ViewMore } from "./components/app/ViewMore";
import { AppSidebar } from "./components/navigation/AppSidebar";
import AppStore from "./store/AppStore";
import SettingsStore from "./store/SettingsStore";

const fetchUser = async () => (await invoke<Hotkey[]>("get_hotkeys"))[0].icon;

function App() {
  const { settings, setGlobalHotkeyEvent, globalHotkeyEvent } = SettingsStore;
  const { sidebarIcons, updateSidebarIcons } = AppStore;
  const [data] = createResource(fetchUser);

  createEffect(async () => {
    await register("CommandOrControl+A", () => {
      console.log("Shortcut triggered");
    });
    // listen to the `click` event and get a function to remove the event listener
    // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
    // console.log("test");
    const unlisten = await listen("click", (event) => {
      console.log(event.payload);
    });
    // emit("click", {
    //   theMessage: "Tauri is awesome!",
    // });
  });

  const sIcon = sidebarIcons().find((icon) => icon.current);

  return (
    <div class="absolute flex h-full w-full overflow-hidden bg-white text-black dark:bg-dark dark:text-white">
      <div class="flex flex-col items-center space-y-7 bg-gray-200 px-3.5 pt-5 dark:bg-dark-light">
        <AppSidebar />
      </div>
      <div class="min-w-0 flex-1">
        <div class="flex w-full justify-between py-1 pl-2">
          <p class="bg-gray-50 text-xs font-semibold text-gray-500 dark:bg-dark-dark dark:text-white ">
            {sIcon?.name?.toLocaleUpperCase()}
          </p>
          {settings()?.synchronize ? (
            <FiGlobe title="online" />
          ) : (
            <BsHddFill title="offline" />
          )}
        </div>
        {sIcon?.name === "Recent Clipboards" && sIcon?.current && (
          <RecentClipboards />
        )}
        {sIcon?.name === "Starred Clipboards" && sIcon?.current && (
          <StarredClipboards />
        )}
        {sIcon?.name === "History" && sIcon?.current && <History />}
        {sIcon?.name === "View more" && sIcon?.current && <ViewMore />}
      </div>
    </div>
  );History
}

export default App;
