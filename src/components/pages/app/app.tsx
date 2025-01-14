import { BsHddFill } from "solid-icons/bs";
import { FiGlobe } from "solid-icons/fi";
import { Show } from "solid-js";
import { AppStore } from "../../../store/app-store";
import { SettingsStore } from "../../../store/settings-store";
import { AppSidebar } from "../../navigation/app-sidebar";
import { useLanguage } from "../../provider/language-provider";
import { ClipboardHistory } from "./clipboard-history";
import { RecentClipboards } from "./recent-clipboards";
import { StarredClipboards } from "./starred-clipboards";
import { ViewMore } from "./view-more";

function App() {
  const { t } = useLanguage();

  return (
    <div class="flex h-full w-full overflow-hidden bg-white text-black dark:bg-dark dark:text-white">
      <div class="flex w-12 flex-col items-center space-y-3 bg-gray-200 pt-2 dark:bg-dark-light">
        <AppSidebar />
      </div>
      <div class="flex h-screen min-w-0 flex-1 flex-col">
        <div class="z-10 flex w-full justify-between overflow-visible px-2 py-1">
          <p class="select-none text-xs font-semibold uppercase text-gray-500 dark:text-white">
            {t(AppStore.getCurrentTab().name)}
          </p>
          <Show when={SettingsStore.settings()?.sync} fallback={<BsHddFill title="offline" />}>
            <FiGlobe title="online" />
          </Show>
        </div>

        <Show when={AppStore.getCurrentTab()?.name === "MAIN.HOTKEY.RECENT_CLIPBOARDS"}>
          <RecentClipboards />
        </Show>

        <Show when={AppStore.getCurrentTab()?.name === "MAIN.HOTKEY.STARRED_CLIPBOARDS"}>
          <StarredClipboards />
        </Show>

        <Show when={AppStore.getCurrentTab()?.name === "MAIN.HOTKEY.HISTORY"}>
          <ClipboardHistory />
        </Show>

        <Show when={AppStore.getCurrentTab()?.name === "MAIN.HOTKEY.VIEW_MORE"}>
          <ViewMore />
        </Show>
      </div>
    </div>
  );
}

export default App;
