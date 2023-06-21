import { Component, For, Show } from "solid-js";
import AppStore from "../../store/AppStore";
import HotkeyStore from "../../store/HotkeyStore";

interface AppSidebarProps {}

export const AppSidebar: Component<AppSidebarProps> = ({}) => {
  const { hotkeys, globalHotkeyEvent, getHotkey } = HotkeyStore;
  const { sidebarIcons, updateSidebarIcons } = AppStore;
  return (
    <Show when={hotkeys().length}>
      <For each={sidebarIcons()}>
        {({ current, Icon, name }) => {
          const currentHotkey = hotkeys()?.find((key) => key.name === name);

          return (
            <div class="relative" title={currentHotkey?.name}>
              <Icon
                class={`${
                  current
                    ? "text-black dark:text-white"
                    : "text-gray-500 dark:text-gray-dark"
                } cursor-pointer text-xl hover:text-black dark:hover:text-white`}
                onClick={() => updateSidebarIcons(name)}
                title={name}
              />
              <Show
                when={getHotkey(currentHotkey!.event) && globalHotkeyEvent()}
              >
                <div class="absolute -left-2 -top-3 rounded-sm bg-zinc-600 px-1 text-[12px] font-semibold">
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
