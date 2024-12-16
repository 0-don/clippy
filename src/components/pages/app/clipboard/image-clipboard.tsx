import { BsImages } from "solid-icons/bs";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../../types";
import { formatBytes } from "../../../../utils/helpers";
import { HotkeyNumber } from "../../../utils/hotkey-number";

interface ImageClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const ImageClipboard: Component<ImageClipboardProps> = (props) => {
  return (
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
  );
};
