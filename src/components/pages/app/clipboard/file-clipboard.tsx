import { VsFileBinary } from "solid-icons/vs";
import { Component } from "solid-js";
import { SettingsStore } from "../../../../store/settings-store";
import { ClipboardFileModel, ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { formatBytes } from "../../../../utils";
import { copyEntry } from "../../../../utils/clipboard-actions";
import { LANGUAGES } from "../../../../utils/constants";
import dayjs from "../../../../utils/dayjs";
import { ClipboardHeader } from "./clipboard-header";

interface FileClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const FileClipboard: Component<FileClipboardProps> = (props) => {
  const fromNow = () =>
    dayjs
      .utc(props.data.clipboard.created_at)
      .locale(SettingsStore.settings()?.language || LANGUAGES[0])
      .fromNow();

  const handleClick = async (e: MouseEvent) => {
    e.stopPropagation();
    await copyEntry(props.data.clipboard.id, ClipboardType.File);
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
      {} as Record<
        string,
        { count: number; size: number; files: ClipboardFileModel[] }
      >,
    );
    return grouped || {};
  };

  const getFileListTitle = () => {
    return props.data.files
      ?.map(
        (file) =>
          `${file.name}${file.extension ? `.${file.extension}` : ""} - ${formatBytes(file.size || 0)}`,
      )
      .join("\n");
  };

  return (
    <button type="button" onClick={handleClick} class="clipboard">
      <ClipboardHeader {...props} Icon={VsFileBinary} />

      <div class="min-w-0 flex-1" title={getFileListTitle()}>
        {props.data.clipboard.name && (
          <p class="w-[calc(100vw-6.5rem)] truncate text-left text-sm font-medium">
            {props.data.clipboard.name}
          </p>
        )}
        {Object.entries(getGroupedFiles()).map(([type, data]) => (
          <span class="flex items-center gap-1">
            <span class="truncate text-sm">
              {data.count} {type}
            </span>
            <span class="text-muted-foreground text-xs">
              {formatBytes(data.size)}
            </span>
          </span>
        ))}

        <div
          class="text-muted-foreground text-left text-xs font-thin"
          title={dayjs.utc(props.data.clipboard.created_at).format()}
        >
          {fromNow()}
        </div>
      </div>
    </button>
  );
};
