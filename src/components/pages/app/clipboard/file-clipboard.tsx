import dayjs from "dayjs";
import { VsFileBinary } from "solid-icons/vs";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { formatBytes } from "../../../../utils/helpers";
import { invokeCommand } from "../../../../utils/tauri";
import { HotkeyNumber } from "../../../utils/hotkey-number";

interface FileClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const FileClipboard: Component<FileClipboardProps> = (props) => {
  const handleClick = async (e: MouseEvent) => {
    e.stopPropagation();
    await invokeCommand(InvokeCommand.CopyClipboard, {
      id: props.data.clipboard.id,
      type: ClipboardType.File,
    });
  };

  const getTotalSize = () => {
    return props.data.files?.reduce((total, file) => total + (file.size || 0), 0) || 0;
  };

  const getFileListTitle = () => {
    return props.data.files
      ?.map((file) => `${file.name}${file.extension ? `.${file.extension}` : ""} - ${formatBytes(file.size || 0)}`)
      .join("\n");
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
              <VsFileBinary class="text-2xl text-zinc-700 dark:text-white" />
              <HotkeyNumber index={props.index} />
            </div>
          </div>
          <div class="mr-4 truncate px-4">
            <div class="flex items-center gap-2" title={getFileListTitle()}>
              <p class="text-sm">
                {props.data.files?.length || 0} file{props.data.files?.length !== 1 ? "s" : ""}
              </p>
              <span class="text-xs text-zinc-500">{formatBytes(getTotalSize())}</span>
            </div>
          </div>
        </div>
        <div class="pl-9 text-left text-xs text-zinc-400">{dayjs.utc(props.data.clipboard.created_date).fromNow()}</div>
      </div>
      <hr class="border-zinc-400 dark:border-zinc-700" />
    </button>
  );
};
