import { BsJournalRichtext } from "solid-icons/bs";
import { FiChevronDown, FiChevronUp, FiEdit3 } from "solid-icons/fi";
import { IoTrashOutline } from "solid-icons/io";
import { TbOutlineSourceCode } from "solid-icons/tb";
import { VsStarFull } from "solid-icons/vs";
import { Component, createSignal, Show } from "solid-js";
import { invokeCommand } from "../../../../lib/tauri";
import { ClipboardStore } from "../../../../store/clipboard-store";
import { ClipboardModel, ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { copyEntry } from "../../../../utils/clipboard-actions";
import { MAX_TEXT_PREVIEW } from "../../../../utils/constants";
import { ClipboardEditor } from "../../../elements/clipboard-editor";
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
  const [editing, setEditing] = createSignal(false);

  const handleDelete = async (id: number) => {
    await invokeCommand(InvokeCommand.DeleteClipboard, { id });
    const remaining = ClipboardStore.clipboards().filter(
      (o) => o.clipboard.id !== id,
    );
    ClipboardStore.setClipboards(remaining);
    if (!remaining.length) await ClipboardStore.resetClipboards();
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
          : o,
      ),
    );
  };

  const handleEdit = (e: MouseEvent) => {
    e.stopPropagation();
    setEditing(true);
  };

  // Refetched rather than patched from the edited string: the backend
  // reclassifies the text (link, hex, rgb, plain) on save, and that type drives
  // the icon and colour swatch.
  const handleSaved = async () => {
    const saved = await invokeCommand(InvokeCommand.GetClipboard, {
      id: props.data.clipboard.id,
    });

    ClipboardStore.setClipboards((prev) =>
      prev.map((o) =>
        o.clipboard.id === props.data.clipboard.id
          ? {
              ...saved,
              text: saved.text
                ? {
                    ...saved.text,
                    data: saved.text.data.slice(0, MAX_TEXT_PREVIEW),
                  }
                : saved.text,
            }
          : o,
      ),
    );
  };

  const handleRtfCopy = async (e: MouseEvent) => {
    e.stopPropagation();
    await copyEntry(props.data.clipboard.id, ClipboardType.Rtf);
  };

  const handleHtmlCopy = async (e: MouseEvent) => {
    e.stopPropagation();
    await copyEntry(props.data.clipboard.id, ClipboardType.Html);
  };

  const canExpand = () =>
    props.data.clipboard.types.includes(ClipboardType.Image) ||
    props.data.clipboard.types.includes(ClipboardType.Text) ||
    props.data.clipboard.types.includes(ClipboardType.Html) ||
    props.data.clipboard.types.includes(ClipboardType.Rtf);

  const handleExpandToggle = (e: MouseEvent) => {
    e.stopPropagation();
    ClipboardStore.toggleExpanded(props.data.clipboard.id);
  };

  return (
    <div class={["group relative", { "bg-muted": props.isSelected }]}>
      <Show when={editing()}>
        <ClipboardEditor
          entry={props.data}
          onClose={() => setEditing(false)}
          onSaved={handleSaved}
        />
      </Show>
      {
        <>
          {/* Actions overlay */}
          <div class="absolute top-0 right-0 bottom-0 z-10 my-1 flex flex-col items-end justify-between">
            <VsStarFull
              onClick={(e) => {
                e.stopPropagation();
                handleStar(props.data.clipboard);
              }}
              title={t("CLIPBOARD.STAR_FAVORITE")}
              class={`${
                props.data.clipboard.star
                  ? "text-yellow-400 dark:text-yellow-300"
                  : "text-foreground hidden"
              } cursor-pointer group-hover:block hover:text-yellow-400 dark:hover:text-yellow-300`}
            />
            <div class="flex items-center gap-1">
              <FiEdit3
                onClick={handleEdit}
                title={t("CLIPBOARD.EDIT_ENTRY")}
                class="text-foreground hidden cursor-pointer group-hover:block hover:text-blue-600 dark:hover:text-blue-400"
              />
              {props.data.rtf && (
                <BsJournalRichtext
                  onClick={handleRtfCopy}
                  title={t("CLIPBOARD.COPY_AS_RTF")}
                  class="text-foreground hidden cursor-pointer group-hover:block hover:text-blue-600 dark:hover:text-blue-400"
                />
              )}
              {props.data.html && (
                <TbOutlineSourceCode
                  onClick={handleHtmlCopy}
                  title={t("CLIPBOARD.COPY_AS_HTML")}
                  class="text-foreground hidden cursor-pointer group-hover:block hover:text-green-600 dark:hover:text-green-400"
                />
              )}
            </div>
            <div class="flex items-center gap-1">
              {canExpand() &&
                (ClipboardStore.isExpanded(props.data.clipboard.id) ? (
                  <FiChevronUp
                    onClick={handleExpandToggle}
                    title={t("CLIPBOARD.COLLAPSE")}
                    class="text-foreground cursor-pointer hover:text-blue-600 dark:hover:text-blue-400"
                  />
                ) : (
                  <FiChevronDown
                    onClick={handleExpandToggle}
                    title={t("CLIPBOARD.EXPAND")}
                    class="text-foreground hidden cursor-pointer group-hover:block hover:text-blue-600 dark:hover:text-blue-400"
                  />
                ))}
              <IoTrashOutline
                onClick={(e) => {
                  e.stopPropagation();
                  handleDelete(props.data.clipboard.id);
                }}
                title={t("CLIPBOARD.DELETE_CLIPBOARD")}
                class="text-foreground hidden cursor-pointer group-hover:block hover:text-red-600 dark:hover:text-red-600"
              />
            </div>
          </div>

          {/* Content rendered by specific clipboard type */}
          {props.data.clipboard.types.includes(ClipboardType.Image) && (
            <ImageClipboard {...props} />
          )}
          {props.data.clipboard.types.includes(ClipboardType.File) && (
            <FileClipboard {...props} />
          )}
          {(props.data.clipboard.types.includes(ClipboardType.Text) ||
            props.data.clipboard.types.includes(ClipboardType.Html) ||
            props.data.clipboard.types.includes(ClipboardType.Rtf)) && (
            <TextClipboard {...props} />
          )}
        </>
      }
    </div>
  );
};
