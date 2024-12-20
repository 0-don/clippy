import dayjs from "dayjs";
import { IoText } from "solid-icons/io";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { invokeCommand } from "../../../../utils/tauri";
import { ClipboardHeader } from "../../../utils/clipboard/clipboard-header";

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
    <button type="button" onClick={handleClick} class="clipboard">
      <ClipboardHeader {...props} Icon={IoText} />

      <div class="min-w-0 flex-1">
        <p class="truncate text-left text-sm" title={data}>
          {data}
        </p>
        <div
          class="text-left text-xs font-thin text-zinc-700 dark:text-zinc-300"
          title={dayjs.utc(props.data.clipboard.created_date).format()}
        >
          {dayjs.utc(props.data.clipboard.created_date).fromNow()}
        </div>
      </div>
    </button>
  );
};
