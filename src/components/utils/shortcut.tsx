import { Component } from "solid-js";
import { HotkeyStore } from "../../store/hotkey-store";
import { Hotkey } from "../../types";
import { useLanguage } from "../provider/language-provider";
import { HotkeyRecorder } from "./hotkey-recorder";

interface ShortcutProps {
  hotkey: Hotkey;
}

export const Shortcut: Component<ShortcutProps> = (props) => {
  const { t } = useLanguage();

  return (
    <div class="flex w-full items-center gap-2 text-sm">
      <div class="w-5 shrink-0">
        <div innerHTML={JSON.parse(props.hotkey.icon)} class="relative" />
      </div>
      <p class="flex-1 truncate">{t(props.hotkey.name)}</p>
      <HotkeyRecorder
        value={props.hotkey}
        icon={false}
        onRecord={(hotkey) =>
          HotkeyStore.updateHotkey({ ...props.hotkey, ...hotkey, status: true })
        }
        onClear={() =>
          HotkeyStore.updateHotkey({
            ...props.hotkey,
            key: "none",
            status: false,
          })
        }
      />
    </div>
  );
};
