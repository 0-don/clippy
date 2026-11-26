import { Dynamic } from "@solidjs/web";
import { IconTypes } from "solid-icons";
import { Component, Show } from "solid-js";
import { HotkeyStore } from "../../../../store/hotkey-store";
import { ClipboardWithRelations } from "../../../../types";

interface ClipboardHeaderProps {
  data: ClipboardWithRelations;
  index: number;
  Icon: IconTypes;
}

export const ClipboardHeader: Component<ClipboardHeaderProps> = (props) => {
  return (
    <div class="relative" title={props.data.clipboard.id.toString()}>
      <Dynamic component={props.Icon} class="text-foreground text-2xl" />
      <Show when={HotkeyStore.globalHotkeyEvent() && props.index + 1 < 10}>
        <div class="bg-card text-card-foreground absolute top-0 left-0 z-50 -mt-1.5 -ml-3 rounded-xs px-1 text-[12px] font-semibold">
          {props.index + 1}
        </div>
      </Show>
    </div>
  );
};
