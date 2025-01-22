import { createRoot, createSignal } from "solid-js";
import { invokeCommand } from "../lib/tauri";
import { ClipboardWhere, ClipboardWithRelations } from "../types";
import { InvokeCommand } from "../types/tauri-invoke";

export const initialWhere: ClipboardWhere = {
  cursor: undefined,
  search: undefined,
  star: undefined,
  img: undefined,
};

function createClipboardStore() {
  const [clipboardRef, setClipboardRef] = createSignal<HTMLDivElement | undefined>();
  const [clipboards, setClipboards] = createSignal<ClipboardWithRelations[]>([]);
  const [where, setWhere] = createSignal<ClipboardWhere>(initialWhere);
  const [hasMore, setHasMore] = createSignal(true);
  const resetWhere = () => setWhere(initialWhere);
  const [selectedIndex, setSelectedIndex] = createSignal(-1);

  const getClipboards = async () => {
    const response = await invokeCommand(InvokeCommand.GetClipboards, where());
    setHasMore(response.has_more);
    return response.clipboards;
  };

  const newClipboard = (clipboard: ClipboardWithRelations) => {
    setClipboards((prev) => {
      const newClipboards = [clipboard, ...prev];
      return newClipboards.sort((a, b) => {
        const dateA = new Date(a.clipboard.created_at).getTime();
        const dateB = new Date(b.clipboard.created_at).getTime();
        return dateB - dateA;
      });
    });
  };

  const init = async () => {
    setWhere(initialWhere);
    const clipboards = await getClipboards();
    setClipboards(clipboards);
    ClipboardStore.clipboardRef()?.scrollTo(0, 0);
  };

  const resetClipboards = async () => {
    setWhere(initialWhere);
    setHasMore(true);
    const clipboards = await getClipboards();
    setClipboards(clipboards);
  };

  const handleKeyDown = async (e: KeyboardEvent) => {
    if (!clipboards().length) return;

    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        const isLastItem = selectedIndex() === clipboards().length - 1;

        // If we're at the last item and there's more data, load more
        if (isLastItem && hasMore()) {
          setWhere((prev) => ({ ...prev, cursor: clipboards().length }));
          const newClipboards = await getClipboards();
          setClipboards((prev) => [...prev, ...newClipboards]);
          // Move to next item after loading more
          setSelectedIndex((prev) => prev + 1);
        } else {
          // Normal navigation if not at last item or no more data
          setSelectedIndex((prev) => (prev < clipboards().length - 1 ? prev + 1 : prev));
        }

        // Ensure selected item is visible
        const nextElement = clipboardRef()?.children[selectedIndex() + 1];
        nextElement?.scrollIntoView({ block: "nearest" });
        break;

      case "ArrowUp":
        e.preventDefault();
        setSelectedIndex((prev) => (prev > 0 ? prev - 1 : prev));
        // Ensure selected item is visible
        const prevElement = clipboardRef()?.children[selectedIndex()];
        prevElement?.scrollIntoView({ block: "nearest" });
        break;

      case "Enter":
        if (selectedIndex() >= 0) {
          const clipboard = clipboards()[selectedIndex()];
          const type = clipboard.clipboard.types[0];
          await invokeCommand(InvokeCommand.CopyClipboard, {
            id: clipboard.clipboard.id,
            type,
          });
        }
        break;
    }
  };

  return {
    clipboards,
    newClipboard,
    setClipboards,
    where,
    setWhere,
    hasMore,
    setHasMore,
    resetClipboards,
    resetWhere,
    getClipboards,
    clipboardRef,
    setClipboardRef,
    init,
    handleKeyDown,
    selectedIndex,
    setSelectedIndex,
  };
}

export const ClipboardStore = createRoot(createClipboardStore);
export type ClipboardStore = ReturnType<typeof createClipboardStore>;
