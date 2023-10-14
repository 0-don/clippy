import { Component, For, Show } from "solid-js";
import AppStore from "../../store/AppStore";
import HotkeyStore from "../../store/HotkeyStore";

interface AppSidebarProps {}

export const AppSidebar: Component<AppSidebarProps> = ({}) => {
  const { hotkeys, globalHotkeyEvent, getHotkey } = HotkeyStore;
  const { setCurrentTab, tabs } = AppStore;
  return (
    <Show when={hotkeys().length}>
      <For each={tabs()}>
        {({ current, Icon, name, id }) => {
          const currentHotkey = hotkeys()?.find((key) => key?.name === name);

          return (
            <div class="relative" title={currentHotkey?.name}>
              <Icon
                class={`${
                  current
                    ? "text-black dark:text-white"
                    : "text-zinc-600 dark:text-gray-dark"
                } cursor-pointer text-xl hover:text-black dark:hover:text-white`}
                onClick={() => setCurrentTab(id)}
                title={name}
              />
              <Show
                when={
                  currentHotkey?.event &&
                  getHotkey(currentHotkey?.event) &&
                  globalHotkeyEvent()
                }
              >
                <div class="absolute -left-2 -top-3 rounded-sm bg-zinc-800 px-1 text-[12px] font-semibold text-white dark:bg-zinc-600">
                  {currentHotkey!.key}
                </div>
              </Show>
            </div>
          );
        }}
      </For>
    </Show>
  );
};
