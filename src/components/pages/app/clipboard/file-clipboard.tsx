import { FiFileText } from "solid-icons/fi";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../../types";
import { formatBytes } from "../../../../utils/helpers";
import { HotkeyNumber } from "../../../utils/hotkey-number";

interface FileClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const FileClipboard: Component<FileClipboardProps> = (props) => {
  return (
    <div class="flex min-w-0">
      <div class="flex items-center">
        <div class="relative" title={props.data.clipboard.id.toString()}>
          <FiFileText class="text-2xl text-zinc-700 dark:text-white" />
          <HotkeyNumber index={props.index} />
        </div>
      </div>
      <div class="mr-4 truncate px-4">
        <div class="flex flex-col">
          {props.data.files?.map((file) => (
            <div class="flex items-center gap-2">
              <p class="text-sm">{file.name || "Unnamed file"}</p>
              <span class="text-xs text-zinc-500">{file.size && formatBytes(file.size)}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
