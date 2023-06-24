import { BsHddFill } from "solid-icons/bs";
import { FiGlobe } from "solid-icons/fi";
import { Show } from "solid-js";
import AppStore from "../../../store/AppStore";
import SettingsStore from "../../../store/SettingsStore";
import { AppSidebar } from "../../navigation/AppSidebar";
import { ClipboardHistory } from "./ClipboardHistory";
import { RecentClipboards } from "./RecentClipboards";
import { StarredClipboards } from "./StarredClipboards";
import { ViewMore } from "./ViewMore";

function App() {
  const { settings } = SettingsStore;
  const { sIcon } = AppStore;

  return (
    <div class="absolute flex h-full w-full overflow-hidden bg-white text-black dark:bg-dark dark:text-white">
      <div class="flex w-12 flex-col items-center space-y-7 bg-gray-200 px-3.5 pt-5 dark:bg-dark-light">
        <AppSidebar />
      </div>
      <div class="min-w-0 flex-1">
        <div class="flex w-full justify-between px-2 py-1">
          <p class="bg-gray-50 text-xs font-semibold text-gray-500 dark:bg-dark-dark dark:text-white ">
            {sIcon()?.name?.toLocaleUpperCase()}
          </p>
          <Show
            when={settings()?.synchronize}
            fallback={<BsHddFill title="offline" />}
          >
            <FiGlobe title="online" />
          </Show>
        </div>
        <Show when={sIcon()?.name === "Recent Clipboards"}>
          <RecentClipboards />
        </Show>

        <Show when={sIcon()?.name === "Starred Clipboards"}>
          <StarredClipboards />
        </Show>

        <Show when={sIcon()?.name === "History"}>
          <ClipboardHistory />
        </Show>

        <Show when={sIcon()?.name === "View more"}>
          <ViewMore />
        </Show>
      </div>
    </div>
  );
}

export default App;