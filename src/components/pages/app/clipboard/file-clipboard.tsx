import dayjs from "dayjs";
import { VsFileBinary } from "solid-icons/vs";
import { Component } from "solid-js";
import { ClipboardFileModel, ClipboardWithRelations } from "../../../../types";
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

  const getGroupedFiles = () => {
    const grouped = props.data.files?.reduce(
      (acc, file) => {
        const type = file.mime_type || "unknown";
        if (!acc[type]) {
          acc[type] = { count: 0, size: 0, files: [] };
        }
        acc[type].count += 1;
        acc[type].size += file.size || 0;
        acc[type].files.push(file);
        return acc;
      },
      {} as Record<string, { count: number; size: number; files: ClipboardFileModel[] }>
    );

    return grouped || {};
  };

  const getFileListTitle = () => {
    return props.data.files
      ?.map((file) => `${file.name}${file.extension ? `.${file.extension}` : ""} - ${formatBytes(file.size || 0)}`)
      .join("\n");
  };

  const groupedFiles = getGroupedFiles();

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
              <div class="flex flex-wrap">
                {Object.entries(groupedFiles).map(([type, data], index) => (
                  <>
                    <span class="flex items-center gap-1">
                      <span class="text-sm">
                        {data.count} {type}
                      </span>
                      <span class="text-xs text-zinc-500">{formatBytes(data.size)}</span>
                    </span>
                  </>
                ))}
              </div>
            </div>
          </div>
        </div>
        <div class="pl-9 text-left text-xs text-zinc-400">{dayjs.utc(props.data.clipboard.created_date).fromNow()}</div>
      </div>
      <hr class="border-zinc-400 dark:border-zinc-700" />
    </button>
  );
};
