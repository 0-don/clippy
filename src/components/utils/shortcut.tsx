import { Component, Show } from "solid-js";
import { HotkeyStore } from "../../store/hotkey-store";
import { Hotkey } from "../../types";
import { GLOBAL_SHORTCUT_KEYS } from "../../utils/constants";
import { CheckBox } from "../elements/checkbox";
import { Dropdown } from "../elements/dropdown";

interface ShortcutProps {
  hotkey: Hotkey;
}

export const Shortcut: Component<ShortcutProps> = (props) => {
  const { updateHotkey } = HotkeyStore;

  return (
    <>
      <div class="flex w-full items-center space-x-2.5 text-sm">
        <div class="w-8">
          <div innerHTML={JSON.parse(props.hotkey.icon)} class="relative" />
        </div>

        <Show when={props.hotkey.status}>
          <CheckBox
            checked={props.hotkey.ctrl}
            onChange={(ctrl) => updateHotkey({ ...props.hotkey, ctrl })}
            text="Ctrl"
          />
          <CheckBox checked={props.hotkey.alt} onChange={(alt) => updateHotkey({ ...props.hotkey, alt })} text="Alt" />
          <CheckBox
            checked={props.hotkey.shift}
            onChange={(shift) => updateHotkey({ ...props.hotkey, shift })}
            text="Shift"
          />
        </Show>
        <Dropdown
          items={GLOBAL_SHORTCUT_KEYS.map((key) => ({ value: key, label: key }))}
          value={props.hotkey.key}
          onChange={(currentKey) => {
            if (typeof currentKey === "number") return;
            updateHotkey({
              ...props.hotkey,
              key: currentKey,
              status: currentKey !== "none",
            });
          }}
        />
        <p class="flex w-full justify-end truncate">{props.hotkey.name}</p>
      </div>
    </>
  );
};
