import { Component, Show } from "solid-js";
import { HotkeyStore } from "../../store/hotkey-store";

export const HotkeyNumber: Component<{ index: number }> = (props) => {
  const { globalHotkeyEvent } = HotkeyStore;

  return (
    <Show when={globalHotkeyEvent() && props.index + 1 < 10}>
      <div class="absolute left-0 top-0 -ml-3 -mt-3 rounded-sm bg-zinc-800 px-1 text-[12px] font-semibold text-white dark:bg-zinc-600">
        {props.index + 1}
      </div>
    </Show>
  );
};
