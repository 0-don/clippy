import { IoTrashOutline } from "solid-icons/io";
import { VsStarFull } from "solid-icons/vs";
import { Component } from "solid-js";
import { ClipboardStore } from "../../../../store/clipboard-store";
import { ClipboardModel, ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { invokeCommand } from "../../../../utils/tauri";
import { FileClipboard } from "./file-clipboard";
import { ImageClipboard } from "./image-clipboard";
import { TextClipboard } from "./text-clipboard";

interface BaseClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const BaseClipboard: Component<BaseClipboardProps> = (props) => {
  const { setClipboards } = ClipboardStore;
  const { clipboard } = props.data;

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

  return (
    <div class="group relative">
      {/* Actions overlay */}
      <div class="absolute bottom-0 right-0 top-0 z-10 m-2 flex w-4 flex-col items-end justify-between">
        <VsStarFull
          onClick={(e) => {
            e.stopPropagation();
            handleStar(clipboard);
          }}
          class={`${
            clipboard.star ? "text-yellow-400 dark:text-yellow-300" : "hidden text-zinc-700"
          } hover:text-yellow-400 group-hover:block dark:text-white dark:hover:text-yellow-300`}
        />
        <IoTrashOutline
          onClick={(e) => {
            e.stopPropagation();
            handleDelete(clipboard.id);
          }}
          class="hidden text-zinc-700 hover:text-red-600 group-hover:block dark:text-white dark:hover:text-red-600"
        />
      </div>

      {/* Content rendered by specific clipboard type */}
      {clipboard.types.includes(ClipboardType.Image) && <ImageClipboard {...props} />}
      {clipboard.types.includes(ClipboardType.File) && <FileClipboard {...props} />}
      {clipboard.types.includes(ClipboardType.Text) && <TextClipboard {...props} />}
    </div>
  );
};
