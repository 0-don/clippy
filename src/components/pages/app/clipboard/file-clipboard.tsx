import { VsFileBinary } from "solid-icons/vs";
import { Component } from "solid-js";
import { ClipboardFileModel, ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { formatBytes } from "../../../../utils/helpers";
import { invokeCommand } from "../../../../utils/tauri";
import { ClipboardFooter } from "../../../utils/clipboard/clipboard-footer";
import { ClipboardHeader } from "../../../utils/clipboard/clipboard-header";

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
      class="group w-full cursor-pointer select-none hover:bg-zinc-200 dark:hover:bg-neutral-700"
    >
      <div class="mt-2 flex gap-2">
        <ClipboardHeader {...props} Icon={VsFileBinary} />
        <div class="flex flex-wrap gap-2" title={getFileListTitle()}>
          {Object.entries(groupedFiles).map(([type, data]) => (
            <span class="flex items-center gap-1">
              <span class="text-sm">
                {data.count} {type}
              </span>
              <span class="text-xs text-zinc-500">{formatBytes(data.size)}</span>
            </span>
          ))}
        </div>
      </div>
      <ClipboardFooter {...props} />
    </button>
  );
};
