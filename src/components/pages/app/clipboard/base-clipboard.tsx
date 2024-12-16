import { IoTrashOutline } from "solid-icons/io";
import { VsStarFull } from "solid-icons/vs";
import { Component, Show } from "solid-js";
import { ClipboardStore } from "../../../../store/clipboard-store";
import { HotkeyStore } from "../../../../store/hotkey-store";
import { ClipboardModel, ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { invokeCommand } from "../../../../utils/tauri";
import { FileClipboard } from "./file-clipboard";
import { HtmlClipboard } from "./html-clipboard";
import { ImageClipboard } from "./image-clipboard";
import { RtfClipboard } from "./rtf-clipboard";
import { TextClipboard } from "./text-clipboard";

interface BaseClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}



export const BaseClipboard: Component<BaseClipboardProps> = (props) => {
  let dbClickTimer: any;
  const { setClipboards } = ClipboardStore;
  const { clipboard } = props.data;

  const handleClick = async (e: MouseEvent) => {
    e.stopPropagation();
    if (e.detail === 1) {
      dbClickTimer = setTimeout(
        async () =>
          await invokeCommand(InvokeCommand.CopyClipboard, {
            id: clipboard.id,
            type: ClipboardType.Text,
          }),
        clipboard.types.includes(ClipboardType.Image) ? 200 : 0
      );
    }
  };

  const handleDoubleClick = async (e: MouseEvent) => {
    clearTimeout(dbClickTimer);
    e.stopPropagation();
    if (!clipboard.types.includes(ClipboardType.Image)) return;
    await invokeCommand(InvokeCommand.SaveClipboardImage, { id: clipboard.id });
  };

  const handleDelete = async (id: number) => {
    if (await invokeCommand(InvokeCommand.DeleteClipboard, { id })) {
      setClipboards((prev) => prev.filter((o) => o.clipboard.id !== id));
    }
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

  const renderClipboardContent = () => {
    switch (
      clipboard.types[0] // Using first type as primary
    ) {
      case ClipboardType.Text:
        return <TextClipboard {...props} />;
      case ClipboardType.Image:
        return <ImageClipboard {...props} />;
      case ClipboardType.Html:
        return <HtmlClipboard {...props} />;
      case ClipboardType.Rtf:
        return <RtfClipboard {...props} />;
      case ClipboardType.File:
        return <FileClipboard {...props} />;
      default:
        return <TextClipboard {...props} />;
    }
  };

  return (
    <button
      type="button"
      class="group relative w-full cursor-pointer select-none px-3 hover:bg-zinc-200 dark:hover:bg-neutral-700"
      onClick={handleClick}
      onDblClick={handleDoubleClick}
    >
      <div class="flex justify-between py-3">
        {renderClipboardContent()}
        {/* Actions section */}
        <div class="absolute bottom-0 right-0 top-0 m-2 flex w-4">
          <div class="flex w-full flex-col items-end justify-between">
            <VsStarFull
              onClick={(e) => {
                e.stopPropagation();
                handleStar(clipboard);
              }}
              class={`${
                clipboard.star ? "text-yellow-400 dark:text-yellow-300" : "hidden text-zinc-700"
              } z-10 hover:text-yellow-400 group-hover:block dark:text-white dark:hover:text-yellow-300`}
            />
            <IoTrashOutline
              onClick={(e) => {
                e.stopPropagation();
                handleDelete(clipboard.id);
              }}
              class="hidden text-zinc-700 hover:text-red-600 group-hover:block dark:text-white dark:hover:text-red-600"
            />
          </div>
        </div>
      </div>
      <hr class="border-zinc-400 dark:border-zinc-700" />
    </button>
  );
};
