import { invoke } from "@tauri-apps/api";
import { FaRegularKeyboard } from "solid-icons/fa";
import { Match, Switch, createEffect, createResource } from "solid-js";
import { Hotkey } from "./@types";
import AppStore from "./store/AppStore";
import SettingsStore from "./store/SettingsStore";

const fetchUser = async () => (await invoke<Hotkey[]>("get_hotkeys"))[0].icon;

function App() {
  const { settings, setGlobalHotkeyEvent, globalHotkeyEvent } = SettingsStore;
  const { sidebarIcons, updateSidebarIcons } = AppStore;

  const [data] = createResource(fetchUser);

  const html = <FaRegularKeyboard />;

  createEffect(() => {
    console.log(data());
  });
  const sIcon = sidebarIcons().find((icon) => icon.current);

  // createEffect(() => {
  //   const setRecentClipboards = window.electron.on(
  //     "recentClipboards",
  //     (sidebarIconName: SidebarIconName) => updateSidebarIcons(sidebarIconName)
  //   );

  //   const setStarredClipboards = window.electron.on(
  //     "starredClipboards",
  //     (sidebarIconName: SidebarIconName) => setSidebarIcon(sidebarIconName)
  //   );

  //   const setHistory = window.electron.on(
  //     "history",
  //     (sidebarIconName: SidebarIconName) => setSidebarIcon(sidebarIconName)
  //   );

  //   const setViewMore = window.electron.on(
  //     "viewMore",
  //     (sidebarIconName: SidebarIconName) => setSidebarIcon(sidebarIconName)
  //   );

  //   const enableHotkey = window.electron.on(
  //     "enableHotkey",
  //     (status: boolean) => {
  //       setSidebarIcon("Recent Clipboards");
  //       setGlobalHotkeyEvent(status);
  //     }
  //   );

  //   return () => {
  //     setRecentClipboards();
  //     setStarredClipboards();
  //     setHistory();
  //     setViewMore();
  //     enableHotkey();
  //   };
  // });

  return (
    <div class="dark:bg-dark absolute flex h-full w-full overflow-hidden bg-white text-black dark:text-white ">
      <Switch fallback={<div>Not Found</div>}>
        <Match when={data.state === "pending" || data.state === "unresolved"}>
          Loading...
        </Match>
        <Match when={data.state === "ready"}>
          <div class="!text-5xl" innerHTML={JSON.parse(data() || "{}")}></div>
          <div class="text-red-500">asd</div>
        </Match>
        <Match when={data.state === "errored"}>
          {JSON.stringify(data.error)}
        </Match>
      </Switch>
      {/* <div class="dark:bg-dark-light flex flex-col items-center space-y-7 bg-gray-200 px-3.5 pt-5">
        <AppSidebar />
      </div>
      <div class="min-w-0 flex-1">
        <div class="flex w-full justify-between py-1 pl-2">
          <p class="dark:bg-dark-dark bg-gray-50 text-xs font-semibold text-gray-500 dark:text-white ">
            {sIcon?.name?.toLocaleUpperCase()}
          </p>
          <FontAwesomeIcon
            icon={settings?.synchronize ? ["fas", "globe"] : ["far", "hdd"]}
            title={settings?.synchronize ? "online" : "offline"}
            class="text-1xl mr-2"
          />
        </div>
        {sIcon?.name === "Recent Clipboards" && sIcon?.current && (
          <RecentClipboards />
        )}
        {sIcon?.name === "Starred Clipboards" && sIcon?.current && (
          <StarredClipboards />
        )}
        {sIcon?.name === "History" && sIcon?.current && <History />}
        {sIcon?.name === "View more" && sIcon?.current && <ViewMore />}
      </div> */}
    </div>
  );
}

export default App;
