import { BsImages } from "solid-icons/bs";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { formatBytes } from "../../../../utils/helpers";
import { invokeCommand } from "../../../../utils/tauri";
import { ClipboardFooter } from "../../../utils/clipboard/clipboard-footer";
import { ClipboardHeader } from "../../../utils/clipboard/clipboard-header";

interface ImageClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const ImageClipboard: Component<ImageClipboardProps> = (props) => {
  let dbClickTimer: any;

  const handleClick = async (e: MouseEvent) => {
    e.stopPropagation();
    if (e.detail === 1) {
      dbClickTimer = setTimeout(async () => {
        await invokeCommand(InvokeCommand.CopyClipboard, {
          id: props.data.clipboard.id,
          type: ClipboardType.Image,
        });
      }, 200);
    }
  };

  const handleDoubleClick = async (e: MouseEvent) => {
    clearTimeout(dbClickTimer);
    e.stopPropagation();
    await invokeCommand(InvokeCommand.SaveClipboardImage, { id: props.data.clipboard.id });
  };

  const imageInfo =
    props.data.image &&
    `${props.data.image.width}x${props.data.image.height} ${formatBytes(Number(props.data.image.size || "0"))}`;

  return (
    <button
      type="button"
      onClick={handleClick}
      onDblClick={handleDoubleClick}
      class="group w-full cursor-pointer select-none hover:bg-zinc-200 dark:hover:bg-neutral-700"
    >
      <div class="mt-2 flex gap-2">
        <ClipboardHeader {...props} Icon={BsImages} />
        {props.data.image?.thumbnail && (
          <img
            src={`data:image/*;base64,${props.data.image.thumbnail}`}
            class="max-h-20 w-[calc(100%-3rem)] rounded-md object-cover"
            alt={imageInfo}
            title={imageInfo}
          />
        )}
      </div>
      <ClipboardFooter {...props} />
    </button>
  );
};
