import { BsJournalRichtext } from "solid-icons/bs";
import { IoTrashOutline } from "solid-icons/io";
import { TbSourceCode } from "solid-icons/tb";
import { VsStarFull } from "solid-icons/vs";
import { Component } from "solid-js";
import { invokeCommand } from "../../../../lib/tauri";
import { ClipboardStore } from "../../../../store/clipboard-store";
import { ClipboardModel, ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { FileClipboard } from "./file-clipboard";
import { ImageClipboard } from "./image-clipboard";
import { TextClipboard } from "./text-clipboard";

interface BaseClipboardProps {
  data: ClipboardWithRelations;
  index: number;
  isSelected: boolean;
}

export const BaseClipboard: Component<BaseClipboardProps> = (props) => {
  const { setClipboards, resetClipboards } = ClipboardStore;
  const { clipboard } = props.data;

  const handleDelete = async (id: number) => {
    await invokeCommand(InvokeCommand.DeleteClipboard, { id });
    setClipboards((prev) => {
      const updated = prev.filter((o) => o.clipboard.id !== id);
      if (!updated.length) resetClipboards();
      return updated;
    });
  };

  const handleStar = async (clipboard: ClipboardModel) => {
    await invokeCommand(InvokeCommand.StarClipboard, {
      id: clipboard.id,
      star: !clipboard.star,
    });
    setClipboards((prev) =>
      prev.map((o) =>
        o.clipboard.id === clipboard.id
          ? {
              ...o,
              clipboard: {
                ...o.clipboard,
                star: !o.clipboard.star,
              },
            }
          : o
      )
    );
  };

  const handleRtfCopy = async (e: MouseEvent) => {
    e.stopPropagation();
    await invokeCommand(InvokeCommand.CopyClipboard, {
      id: clipboard.id,
      type: ClipboardType.Rtf,
    });
  };

  const handleHtmlCopy = async (e: MouseEvent) => {
    e.stopPropagation();
    await invokeCommand(InvokeCommand.CopyClipboard, {
      id: clipboard.id,
      type: ClipboardType.Html,
    });
  };

  return (
    <div class={`group relative ${props.isSelected ? "bg-zinc-100 dark:bg-neutral-600" : ""}`}>
      {/* Actions overlay */}
      <div class="absolute bottom-0 right-0 top-0 z-10 my-1 mr-0.5 flex flex-col items-end justify-between">
        <VsStarFull
          onClick={(e) => {
            e.stopPropagation();
            handleStar(clipboard);
          }}
          title="Star"
          class={`${
            clipboard.star ? "text-yellow-400 dark:text-yellow-300" : "hidden text-zinc-700"
          } cursor-pointer text-lg hover:text-yellow-400 group-hover:block dark:text-white dark:hover:text-yellow-300`}
        />
        <div class="flex items-center gap-1">
          {props.data.rtf && (
            <BsJournalRichtext
              onClick={handleRtfCopy}
              title="Copy as RTF"
              class="hidden cursor-pointer text-lg text-zinc-700 hover:text-blue-600 group-hover:block dark:text-white dark:hover:text-blue-400"
            />
          )}
          {props.data.html && (
            <TbSourceCode
              onClick={handleHtmlCopy}
              title="Copy as HTML"
              class="hidden cursor-pointer text-lg text-zinc-700 hover:text-green-600 group-hover:block dark:text-white dark:hover:text-green-400"
            />
          )}
        </div>
        <IoTrashOutline
          onClick={(e) => {
            e.stopPropagation();
            handleDelete(clipboard.id);
          }}
          title="Delete"
          class="hidden cursor-pointer text-lg text-zinc-700 hover:text-red-600 group-hover:block dark:text-white dark:hover:text-red-600"
        />
      </div>

      {/* Content rendered by specific clipboard type */}
      {clipboard.types.includes(ClipboardType.Image) && <ImageClipboard {...props} />}
      {clipboard.types.includes(ClipboardType.File) && <FileClipboard {...props} />}
      {(clipboard.types.includes(ClipboardType.Text) ||
        clipboard.types.includes(ClipboardType.Html) ||
        clipboard.types.includes(ClipboardType.Rtf)) && <TextClipboard {...props} />}
    </div>
  );
};
