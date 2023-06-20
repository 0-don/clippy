import { BsBellFill } from "solid-icons/bs";
import { FiMoon } from "solid-icons/fi";
import { HiSolidCog8Tooth } from "solid-icons/hi";
import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { VsRocket } from "solid-icons/vs";
import { Component, Show } from "solid-js";
import SettingsStore from "../../../store/SettingsStore";
import SwitchField from "../../elements/SwitchField";
import { TextBlock } from "../../elements/TextBlock";
import { DarkMode } from "../../utils/DarkMode";
import { IconToString } from "../../utils/IconToString";
import { Shortcut } from "../../utils/Shortcut";

interface SettingsGeneralProps {}

export const SettingsGeneral: Component<SettingsGeneralProps> = ({}) => {
  const { getHotkey, settings, updateSettings } = SettingsStore;

  return (
    <>
      <IconToString />
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
            <SwitchField
              checked={settings()?.startup || false}
              onChange={(check: boolean) =>
                updateSettings({ ...settings()!, startup: check })
              }
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <BsBellFill />
            <h6 class="text-sm">Show desktop notifications.</h6>
          </div>
          <div>
            <SwitchField
              checked={settings()?.notification || false}
              onChange={(check: boolean) =>
                updateSettings({ ...settings()!, notification: check })
              }
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <FiMoon class="text-white" />
            <h6 class="text-sm">Switch Theme.</h6>
          </div>
          <div>
            <DarkMode />
          </div>
        </div>
      </TextBlock>
    </>
  );
};
