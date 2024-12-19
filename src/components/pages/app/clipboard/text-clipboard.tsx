import { IoText } from "solid-icons/io";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { invokeCommand } from "../../../../utils/tauri";
import { ClipboardFooter } from "../../../utils/clipboard/clipboard-footer";
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
    <button
      type="button"
      onClick={handleClick}
      class="group w-full cursor-pointer select-none hover:bg-zinc-200 dark:hover:bg-neutral-700"
    >
      <div class="mt-2 flex gap-2">
        <ClipboardHeader {...props} Icon={IoText} />

        <p class="truncate text-sm" title={data}>
          {data}
        </p>
      </div>
      <ClipboardFooter {...props} />
    </button>
  );
};
