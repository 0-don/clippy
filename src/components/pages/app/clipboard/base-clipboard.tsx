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
import { useLanguage } from "../../../provider/language-provider";
import { FileClipboard } from "./file-clipboard";
import { ImageClipboard } from "./image-clipboard";
import { TextClipboard } from "./text-clipboard";

interface BaseClipboardProps {
  data: ClipboardWithRelations;
  index: number;
  isSelected: boolean;
}

export const BaseClipboard: Component<BaseClipboardProps> = (props) => {
  const { t } = useLanguage();

  const handleDelete = async (id: number) => {
    await invokeCommand(InvokeCommand.DeleteClipboard, { id });
    ClipboardStore.setClipboards((prev) => {
      const updated = prev.filter((o) => o.clipboard.id !== id);
      if (!updated.length) ClipboardStore.resetClipboards();
      return updated;
    });
  };

  const handleStar = async (clipboard: ClipboardModel) => {
    await invokeCommand(InvokeCommand.StarClipboard, {
      id: clipboard.id,
      star: !clipboard.star,
    });
    ClipboardStore.setClipboards((prev) =>
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
      id: props.data.clipboard.id,
      type: ClipboardType.Rtf,
    });
  };

  const handleHtmlCopy = async (e: MouseEvent) => {
    e.stopPropagation();
    await invokeCommand(InvokeCommand.CopyClipboard, {
      id: props.data.clipboard.id,
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
            handleStar(props.data.clipboard);
          }}
          title={t("CLIPBOARD.STAR_FAVORITE")}
          class={`${
            props.data.clipboard.star ? "text-yellow-400 dark:text-yellow-300" : "hidden text-zinc-700"
          } cursor-pointer hover:text-yellow-400 group-hover:block dark:text-white dark:hover:text-yellow-300`}
        />
        <div class="flex items-center gap-1">
          {props.data.rtf && (
            <BsJournalRichtext
              onClick={handleRtfCopy}
              title={t("CLIPBOARD.COPY_AS_RTF")}
              class="hidden cursor-pointer text-zinc-700 hover:text-blue-600 group-hover:block dark:text-white dark:hover:text-blue-400"
            />
          )}
          {props.data.html && (
            <TbSourceCode
              onClick={handleHtmlCopy}
              title={t("CLIPBOARD.COPY_AS_HTML")}
              class="hidden cursor-pointer text-zinc-700 hover:text-green-600 group-hover:block dark:text-white dark:hover:text-green-400"
            />
          )}
        </div>
        <IoTrashOutline
          onClick={(e) => {
            e.stopPropagation();
            handleDelete(props.data.clipboard.id);
          }}
          title={t("CLIPBOARD.DELETE_CLIPBOARD")}
          class="hidden cursor-pointer text-zinc-700 hover:text-red-600 group-hover:block dark:text-white dark:hover:text-red-600"
        />
      </div>

      {/* Content rendered by specific clipboard type */}
      {props.data.clipboard.types.includes(ClipboardType.Image) && <ImageClipboard {...props} />}
      {props.data.clipboard.types.includes(ClipboardType.File) && <FileClipboard {...props} />}
      {(props.data.clipboard.types.includes(ClipboardType.Text) ||
        props.data.clipboard.types.includes(ClipboardType.Html) ||
        props.data.clipboard.types.includes(ClipboardType.Rtf)) && <TextClipboard {...props} />}
    </div>
  );
};
