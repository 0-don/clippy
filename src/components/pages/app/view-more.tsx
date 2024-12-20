import { Component, Show } from "solid-js";
import { HotkeyStore } from "../../../store/hotkey-store";
import { SettingsStore } from "../../../store/settings-store";
import { Hotkey } from "../../../types";
import { WebWindow } from "../../../types/enums";
import { ViewMoreName } from "../../../utils/constants";
import { Toggle } from "../../elements/toggle";

interface ViewMoreProps {}

export const ViewMore: Component<ViewMoreProps> = ({}) => {
  const createButton = (name: ViewMoreName, callback: () => void) => {
    const hotkey = HotkeyStore.hotkeys().find((key) => key.name === name) as Hotkey;

    return (
      <button
        type="button"
        class="w-full cursor-pointer select-none px-3 hover:bg-zinc-200 dark:hover:bg-neutral-700"
        onClick={(e) => {
          e.preventDefault();
          e.stopPropagation();
          callback();
        }}
      >
        <div class="flex items-center justify-between py-4">
          <div class="flex items-center">
            <div class="relative">
              <div innerHTML={JSON.parse(hotkey.icon)} class="text-2xl" />
              <Show when={HotkeyStore.globalHotkeyEvent() && hotkey.status}>
                <div class="absolute left-0 top-0 -ml-2 -mt-3 rounded-sm bg-zinc-800 px-1 text-[12px] font-semibold text-white dark:bg-zinc-600">
                  {hotkey.key}
                </div>
              </Show>
            </div>
            <p class="px-4 text-base font-semibold">{name}</p>
          </div>
          {name === "Sync Clipboard History" && (
            <Toggle checked={SettingsStore.settings()?.synchronize} onChange={async () => {}} />
          )}
        </div>
        <hr class="border-zinc-700" />
      </button>
    );
  };

  return (
    <>
      {/* Sync Clipboard History  */}
      {createButton("Sync Clipboard History", SettingsStore.syncClipboard)}
      {/* Settings */}
      {createButton("Settings", () => SettingsStore.openWindow(WebWindow.Settings))}
      {/* About */}
      {createButton("About", () => SettingsStore.openWindow(WebWindow.About))}
      {/* Exit */}
      {createButton("Exit", SettingsStore.exitApp)}
    </>
  );
};
