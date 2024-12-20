import { CgDisplayFlex } from "solid-icons/cg";
import { FiMoon } from "solid-icons/fi";
import { HiSolidCog8Tooth } from "solid-icons/hi";
import { IoLanguageOutline } from "solid-icons/io";
import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { VsRocket } from "solid-icons/vs";
import { Component, Show } from "solid-js";
import { HotkeyStore } from "../../../store/hotkey-store";
import { SettingsStore } from "../../../store/settings-store";
import { HotkeyEvent, Language } from "../../../types/enums";
import { Dropdown } from "../../elements/dropdown";
import { Input } from "../../elements/input";
import { TextBlock } from "../../elements/text-block";
import { Toggle } from "../../elements/toggle";
import { DarkMode } from "../../utils/dark-mode";
import { Shortcut } from "../../utils/shortcut";

interface SettingsGeneralProps {}

export const SettingsGeneral: Component<SettingsGeneralProps> = ({}) => {
  return (
    <Show when={SettingsStore.settings()}>
      <TextBlock Icon={RiDeviceKeyboardFill} title="Keyboard shortcut">
        <div class="mb-2 flex items-center space-x-2 px-5 pb-2.5">
          <Show when={HotkeyStore.getHotkey(HotkeyEvent.WindowDisplayToggle)}>
            {(hotkey) => <Shortcut hotkey={hotkey()} />}
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
              checked={SettingsStore.settings()?.startup}
              onChange={async (check: boolean) =>
                SettingsStore.updateSettings({ ...SettingsStore.settings()!, startup: check })
              }
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <IoLanguageOutline />
            <h6 class="text-sm">Change language</h6>
          </div>

          <Dropdown
            className="w-16"
            items={Object.entries(Language).map(([key, value]) => ({ value: value, label: key }))}
            value={SettingsStore.settings()!.language}
            onChange={(language) => {
              SettingsStore.updateSettings({ ...SettingsStore.settings()!, language: language as Language });
            }}
          />
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <CgDisplayFlex />
            <h6 class="text-sm">Display Scale</h6>
          </div>

          <Input
            className="w-16"
            value={SettingsStore.settings()!.display_scale.toString()}
            onChange={(display_scale) => {
              SettingsStore.updateSettings({ ...SettingsStore.settings()!, display_scale: parseInt(display_scale) });
            }}
          />
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
