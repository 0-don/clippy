import { Component, Show } from "solid-js";
import { HotkeyStore } from "../../../store/hotkey-store";
import { SettingsStore } from "../../../store/settings-store";
import { Hotkey } from "../../../types";
import { WebWindow } from "../../../types/enums";
import { ViewMoreName } from "../../../utils/constants";
import { Toggle } from "../../elements/toggle";
import { useLanguage } from "../../provider/language-provider";

interface ViewMoreProps {}

export const ViewMore: Component<ViewMoreProps> = ({}) => {
  const { t } = useLanguage();

  const createButton = (name: ViewMoreName, callback: () => void) => {
    const hotkey = HotkeyStore.hotkeys().find(
      (key) => key.name === name,
    ) as Hotkey;

    return (
      <button
        type="button"
        class="w-full cursor-pointer px-3 select-none hover:bg-muted"
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
                <div class="absolute top-0 left-0 -mt-3 -ml-2 rounded-xs bg-card px-1 text-[12px] font-semibold text-card-foreground">
                  {hotkey.key}
                </div>
              </Show>
            </div>
            <p class="px-4 text-base font-semibold">{t(name)}</p>
          </div>
          {name === "MAIN.HOTKEY.SYNC_CLIPBOARD_HISTORY" && (
            <Toggle
              checked={SettingsStore.settings()?.sync}
              onChange={async () => {}}
            />
          )}
        </div>
        <hr class="border-border" />
      </button>
    );
  };

  return (
    <>
      {/* Sync Clipboard History  */}
      {createButton(
        "MAIN.HOTKEY.SYNC_CLIPBOARD_HISTORY",
        SettingsStore.syncAuthenticateToggle,
      )}
      {/* Settings */}
      {createButton("MAIN.HOTKEY.SETTINGS", () =>
        SettingsStore.openWindow(WebWindow.Settings),
      )}
      {/* About */}
      {createButton("MAIN.HOTKEY.ABOUT", () =>
        SettingsStore.openWindow(WebWindow.About),
      )}
      {/* Exit */}
      {createButton("MAIN.HOTKEY.EXIT", SettingsStore.exitApp)}
    </>
  );
};
