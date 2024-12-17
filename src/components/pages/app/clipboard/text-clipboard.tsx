import { IoText } from "solid-icons/io";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../../types";
import { HotkeyNumber } from "../../../utils/hotkey-number";
import dayjs from "dayjs";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { invokeCommand } from "../../../../utils/tauri";

interface TextClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const TextClipboard: Component<TextClipboardProps> = (props) => {
  const handleClick = async (e: MouseEvent) => {
    e.stopPropagation();
    await invokeCommand(InvokeCommand.CopyClipboard, {
      id: props.data.clipboard.id,
      type: ClipboardType.Text,
    });
  };

  return (
    <button
      type="button"
      onClick={handleClick}
      class="group w-full cursor-pointer select-none px-3 hover:bg-zinc-200 dark:hover:bg-neutral-700"
    >
      <div class="px-0.5 py-3">
        <div class="flex min-w-0">
          <div class="flex items-center">
            <div class="relative" title={props.data.clipboard.id.toString()}>
              <IoText class="text-2xl text-zinc-700 dark:text-white" />
              <HotkeyNumber index={props.index} />
            </div>
          </div>
          <div class="mr-4 truncate px-4">
            <div class="flex" title={props.data.text?.data || ""}>
              <p class="text-sm">{props.data.text?.data || " "}</p>
            </div>
          </div>
        </div>
        <div class="pl-9 text-left text-xs text-zinc-400">{dayjs.utc(props.data.clipboard.created_date).fromNow()}</div>
      </div>
      <hr class="border-zinc-400 dark:border-zinc-700" />
    </button>
  );
};
