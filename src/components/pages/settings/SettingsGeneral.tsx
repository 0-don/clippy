import { BsBellFill } from "solid-icons/bs";
import { FiMoon } from "solid-icons/fi";
import { HiSolidCog8Tooth } from "solid-icons/hi";
import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { VsRocket } from "solid-icons/vs";
import { Component, Show } from "solid-js";
import HotkeyStore from "../../../store/HotkeyStore";
import SettingsStore from "../../../store/SettingsStore";
import { TextBlock } from "../../elements/TextBlock";
import { Toggle } from "../../elements/Toggle";
import { DarkMode } from "../../utils/DarkMode";
import { Shortcut } from "../../utils/Shortcut";

interface SettingsGeneralProps {}

export const SettingsGeneral: Component<SettingsGeneralProps> = ({}) => {
  const { settings, updateSettings } = SettingsStore;
  const { getHotkey } = HotkeyStore;

  return (
    <Show when={settings()}>
      <TextBlock Icon={RiDeviceKeyboardFill} title="Keyboard shortcut">
        <div class="mb-2 flex items-center space-x-2 px-5 pb-2.5">
          <Show when={getHotkey("window_display_toggle")}>
            <Shortcut hotkey={getHotkey("window_display_toggle")!} />
          </Show>
        </div>
      </TextBlock>

      <TextBlock Icon={HiSolidCog8Tooth} title="System">
        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <VsRocket />
            <h6 class="text-sm">Start Clippy on system startup.</h6>
          </div>
          <div>
            <Toggle
              checked={settings()?.startup}
              onChange={async (check: boolean) => updateSettings({ ...settings()!, startup: check })}
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <BsBellFill />
            <h6 class="text-sm">Show desktop notifications.</h6>
          </div>
          <div>
            <Toggle
              checked={settings()?.notification}
              onChange={(check: boolean) => updateSettings({ ...settings()!, notification: check })}
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <FiMoon class="dark:text-white" />
            <h6 class="text-sm">Switch Theme.</h6>
          </div>
          <div>
            <DarkMode />
          </div>
        </div>
      </TextBlock>
    </Show>
  );
};
