import { Component, Show } from "solid-js";
import { Hotkey } from "../../../@types";
import HotkeyStore from "../../../store/HotkeyStore";
import SettingsStore from "../../../store/SettingsStore";
import { ViewMoreName } from "../../../utils/constants";
import { createAboutWindow, createSettingsWindow } from "../../../utils/helpers";
import { Toggle } from "../../elements/Toggle";
import { exit } from "@tauri-apps/plugin-process";

interface ViewMoreProps {}

export const ViewMore: Component<ViewMoreProps> = ({}) => {
  const { settings, syncClipboard } = SettingsStore;
  const { hotkeys, globalHotkeyEvent } = HotkeyStore;

  const createButton = (name: ViewMoreName, onClick: () => void) => {
    const hotkey = hotkeys().find((key) => key.name === name) as Hotkey;

    return (
      <button
        type="button"
        class="w-full cursor-pointer select-none px-3 hover:bg-zinc-200 dark:hover:bg-neutral-700"
        onClick={onClick}
      >
        <div class="flex items-center justify-between py-4">
          <div class="flex items-center ">
            <div class="relative">
              <div innerHTML={JSON.parse(hotkey.icon)} class="text-2xl" />
              <Show when={globalHotkeyEvent() && hotkey.status}>
                <div class="absolute left-0 top-0 -ml-2 -mt-3 rounded-sm bg-zinc-800 px-1 text-[12px] font-semibold text-white dark:bg-zinc-600">
                  {hotkey.key}
                </div>
              </Show>
            </div>
            <p class="px-4 text-base font-semibold">{name}</p>
          </div>
          {name === "Sync Clipboard History" && <Toggle checked={settings()?.synchronize} onChange={async () => {}} />}
        </div>
        <hr class="border-zinc-700" />
      </button>
    );
  };

  return (
    <>
      {/* Sync Clipboard History  */}
      {createButton("Sync Clipboard History", syncClipboard)}
      {/* Preferences */}
      {createButton("Preferences", createSettingsWindow)}
      {/* About */}
      {createButton("About", createAboutWindow)}
      {/* Exit */}
      {createButton("Exit", async () => await exit(1))}
    </>
  );
};
