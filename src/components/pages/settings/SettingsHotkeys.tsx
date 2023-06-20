import { FaSolidKeyboard } from "solid-icons/fa";
import { Component, For } from "solid-js";
import SettingsStore from "../../../store/SettingsStore";
import { TextBlock } from "../../elements/TextBlock";
import { Shortcut } from "../../utils/Shortcut";

interface SettingsHotkeysProps {}

export const SettingsHotkeys: Component<SettingsHotkeysProps> = ({}) => {
  const { hotkeys } = SettingsStore;

  return (
    <>
      <TextBlock Icon={FaSolidKeyboard} title="Change your Hotkeys">
        <div class="h-64 overflow-auto px-5">
          <For each={hotkeys()}>
            {(hotkey, index) => {
              console.log(hotkey.event)
              return (
                <>
                  <div class="flex items-center px-0.5 py-4">
                    <Shortcut hotkey={hotkey} />
                  </div>
                  {hotkeys().length !== index() + 1 && (
                    <hr class="border-zinc-700" />
                  )}
                </>
              );
            }}
          </For>
        </div>
      </TextBlock>
    </>
  );
};
