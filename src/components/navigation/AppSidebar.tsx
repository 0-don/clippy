import { Component } from "solid-js";
import AppStore from "../../store/AppStore";
import SettingsStore from "../../store/SettingsStore";

interface AppSidebarProps {}

export const AppSidebar: Component<AppSidebarProps> = ({}) => {
  const { hotkeys, globalHotkeyEvent } = SettingsStore;
  const { sidebarIcons, updateSidebarIcons } = AppStore;
  return (
    <>
      {sidebarIcons().map(({ current, Icon, name }) => {
        const currentHotkey = hotkeys()?.find((key) => key.name === name);
        return (
          <div class="relative">
            <Icon
              class={`${
                current
                  ? "text-black dark:text-white"
                  : "dark:text-gray-dark text-gray-500"
              } cursor-pointer  hover:text-black dark:hover:text-white`}
              onClick={() => updateSidebarIcons(name)}
              title={name}
            />
            {currentHotkey?.status && globalHotkeyEvent() && (
              <div class="absolute left-0 top-0 -ml-3 -mt-3 rounded-sm bg-zinc-600 px-1 text-[12px] font-semibold">
                {currentHotkey.key}
              </div>
            )}
          </div>
        );
      })}
    </>
  );
};
