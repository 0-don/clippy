import { VsFileBinary } from "solid-icons/vs";
import { Component, createEffect, createSignal } from "solid-js";
import { invokeCommand } from "../../../../lib/tauri";
import { SettingsStore } from "../../../../store/settings-store";
import { ClipboardFileModel, ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { formatBytes } from "../../../../utils";
import { LANGUAGES } from "../../../../utils/constants";
import dayjs from "../../../../utils/dayjs";
import { ClipboardHeader } from "./clipboard-header";

interface FileClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const FileClipboard: Component<FileClipboardProps> = (props) => {
  const [fromNowString, setFromNowString] = createSignal(dayjs.utc(props.data.clipboard.created_at).fromNow());

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

  createEffect(() => {
    dayjs.locale(SettingsStore.settings()?.language || LANGUAGES[0]);
    setFromNowString(dayjs.utc(props.data.clipboard.created_at).fromNow());
  });

  const groupedFiles = getGroupedFiles();

  return (
    <button type="button" onClick={handleClick} class="clipboard">
      <ClipboardHeader {...props} Icon={VsFileBinary} />

      <div class="min-w-0 flex-1">
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
        <div
          class="text-left text-xs font-thin text-zinc-700 dark:text-zinc-300"
          title={dayjs.utc(props.data.clipboard.created_at).format()}
        >
          {fromNowString()}
        </div>
      </div>
    </button>
  );
};
