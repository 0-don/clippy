import { IconTypes } from "solid-icons";
import { Component, Show } from "solid-js";
import { HotkeyStore } from "../../../store/hotkey-store";
import { ClipboardWithRelations } from "../../../types";

interface ClipboardHeaderProps {
  data: ClipboardWithRelations;
  index: number;
  Icon: IconTypes;
}

export const ClipboardHeader: Component<ClipboardHeaderProps> = (props) => {
  const { globalHotkeyEvent } = HotkeyStore;

  return (
    <div class="relative" title={props.data.clipboard.id.toString()}>
      <props.Icon class="text-2xl text-zinc-700 dark:text-white" />
      <Show when={globalHotkeyEvent() && props.index + 1 < 10}>
        <div class="absolute left-0 top-0 z-50 -ml-3 -mt-1.5 rounded-sm bg-zinc-800 px-1 text-[12px] font-semibold text-white dark:bg-zinc-600">
          {props.index + 1}
        </div>
      </Show>
    </div>
  );
};
