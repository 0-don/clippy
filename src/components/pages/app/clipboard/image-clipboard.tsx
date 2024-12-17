import { BsImages } from "solid-icons/bs";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../../types";
import { formatBytes } from "../../../../utils/helpers";
import { HotkeyNumber } from "../../../utils/hotkey-number";
import dayjs from "dayjs";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { invokeCommand } from "../../../../utils/tauri";

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

  return (
    <button
      type="button"
      onClick={handleClick}
      onDblClick={handleDoubleClick}
      class="group w-full cursor-pointer select-none px-3 hover:bg-zinc-200 dark:hover:bg-neutral-700"
    >
      <div class="px-0.5 py-3">
        <div class="flex min-w-0">
          <div class="flex items-center">
            <div class="relative" title={props.data.clipboard.id.toString()}>
              <BsImages class="text-2xl text-zinc-700 dark:text-white" />
              <HotkeyNumber index={props.index} />
            </div>
          </div>
          <div class="mr-4 truncate px-4">
            {props.data.image?.thumbnail && (
              <img
                src={`data:image/*;base64,${props.data.image.thumbnail}`}
                width={props.data.image.width || 0}
                height={props.data.image.height || 0}
                class="max-h-52"
                alt={`${props.data.image.width}x${props.data.image.height} ${props.data.image.size}`}
                title={`${props.data.image.width}x${props.data.image.height} ${formatBytes(Number(props.data.image.size || "0"))}`}
              />
            )}
          </div>
        </div>
        <div class="pl-9 text-left text-xs text-zinc-400">{dayjs.utc(props.data.clipboard.created_date).fromNow()}</div>
      </div>
      <hr class="border-zinc-400 dark:border-zinc-700" />
    </button>
  );
};
