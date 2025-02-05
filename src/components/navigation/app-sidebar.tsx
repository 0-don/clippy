import { Component, For, Show } from "solid-js";
import { AppStore } from "../../store/app-store";
import { HotkeyStore } from "../../store/hotkey-store";
import { useLanguage } from "../provider/language-provider";

interface AppSidebarProps {}

export const AppSidebar: Component<AppSidebarProps> = ({}) => {
  const { t } = useLanguage();

  return (
    <Show when={HotkeyStore.hotkeys().length}>
      <For each={AppStore.tabs()}>
        {({ current, Icon, name, id }) => {
          const currentHotkey = HotkeyStore.hotkeys()?.find((key) => key?.name === name);

          return (
            <div
              class={`${
                current ? "text-black dark:text-white" : "text-zinc-600 dark:text-gray-dark"
              } relative flex h-6 w-full cursor-pointer select-none items-center justify-center py-5 text-xl hover:text-black dark:hover:text-white`}
              title={t(currentHotkey!.name)}
              onClick={() => AppStore.changeTab(id)}
            >
              <Icon title={t(name)} />
              <Show
                when={
                  currentHotkey?.event && HotkeyStore.getHotkey(currentHotkey?.event) && HotkeyStore.globalHotkeyEvent()
                }
              >
                <div class="absolute -top-0.5 left-1 rounded-xs bg-zinc-800 px-1 py-1 text-xs font-semibold text-white dark:bg-zinc-600">
                  {currentHotkey!.key}
                </div>
              </Show>
            </div>
          );
        }}
      </For>
      {/* {import.meta.env.DEV && (
        <button onClick={() => invokeCommand(InvokeCommand.AuthGoogleDrive)} class="">
          AUTH
        </button>
      )} */}
    </Show>
  );
};
