import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { Component, For } from "solid-js";
import { HotkeyStore } from "../../../store/hotkey-store";
import { TextBlock } from "../../elements/text-block";
import { Shortcut } from "../../utils/shortcut";

interface SettingsHotkeysProps {}

export const SettingsHotkeys: Component<SettingsHotkeysProps> = ({}) => {
  return (
    <TextBlock Icon={RiDeviceKeyboardFill} title="Change your Hotkeys">
      <div class="h-64 overflow-auto px-5">
        <For each={HotkeyStore.hotkeys()}>
          {(hotkey, index) => (
            <>
              <div class="flex items-center px-0.5 py-4">
                <Shortcut hotkey={hotkey} />
              </div>
              {HotkeyStore.hotkeys().length !== index() + 1 && <hr class="border-zinc-700" />}
            </>
          )}
        </For>
      </div>
    </TextBlock>
  );
};
