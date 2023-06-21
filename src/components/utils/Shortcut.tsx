import { Component, Show } from "solid-js";
import { Hotkey } from "../../@types";
import SettingsStore from "../../store/SettingsStore";
import { GLOBAL_SHORTCUT_KEYS } from "../../utils/constants";
import { CheckBox } from "../elements/CheckBox";
import { Dropdown } from "../elements/Dropdown";

interface ShortcutProps {
  hotkey: Hotkey;
}

export const Shortcut: Component<ShortcutProps> = ({ hotkey }) => {
  const { updateHotkey } = SettingsStore;
  const { icon, status, ctrl, alt, shift, key, name } = hotkey;

  return (
    <>
      <div class="flex w-full items-center space-x-2.5 text-sm ">
        <div class="w-8">
          <div innerHTML={JSON.parse(icon)} class="relative" />
        </div>

        <Show when={status}>
          <CheckBox
            checked={ctrl}
            onChange={() => updateHotkey({ ...hotkey, ctrl: !ctrl })}
            text="Ctrl"
          />
          <CheckBox
            checked={alt}
            onChange={() => updateHotkey({ ...hotkey, alt: !alt })}
            text="Alt"
          />
          <CheckBox
            checked={shift}
            onChange={() => updateHotkey({ ...hotkey, shift: !shift })}
            text="Shift"
          />
        </Show>
        <Dropdown
          items={GLOBAL_SHORTCUT_KEYS as unknown as string[]}
          value={key}
          onChange={(currentKey) => {
            if (typeof currentKey === "number") return;
            updateHotkey({
              ...hotkey,
              key: currentKey,
              status: currentKey !== "none",
            });
          }}
        />
        <p class="flex w-full justify-end truncate">{name}</p>
      </div>
    </>
  );
};
