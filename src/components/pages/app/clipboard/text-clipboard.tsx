import dayjs from "dayjs";
import { IoText } from "solid-icons/io";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { invokeCommand } from "../../../../utils/tauri";
import { HotkeyNumber } from "../../../utils/hotkey-number";

interface TextClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const TextClipboard: Component<TextClipboardProps> = (props) => {
  let type = ClipboardType.Text;
  let data = props.data.text?.data;

  if (!props.data.text?.data && props.data.html?.data) {
    type = ClipboardType.Html;
    data = props.data.html.data;
  }
  if (!props.data.text?.data && props.data.rtf?.data) {
    type = ClipboardType.Rtf;
    data = props.data.rtf.data;
  }

  const handleClick = async (e: MouseEvent) => {
    e.stopPropagation();
    await invokeCommand(InvokeCommand.CopyClipboard, {
      id: props.data.clipboard.id,
      type,
    });
  };

  return (
    <button
      type="button"
      onClick={handleClick}
      class="group w-full cursor-pointer select-none hover:bg-zinc-200 dark:hover:bg-neutral-700"
    >
      <div class="mt-2 flex gap-2">
        <div class="relative ml-3.5" title={props.data.clipboard.id.toString()}>
          <IoText class="text-2xl text-zinc-700 dark:text-white" />
          <HotkeyNumber index={props.index} />
        </div>

        <p class="truncate text-sm" title={data}>
          {data}
        </p>
      </div>
      <div class="mb-1 ml-10 text-left text-xs text-zinc-400">
        {dayjs.utc(props.data.clipboard.created_date).fromNow()}
      </div>
      <hr class="border-zinc-400 dark:border-zinc-700" />
    </button>
  );
};
