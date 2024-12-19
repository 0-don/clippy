import { createRoot, createSignal } from "solid-js";
import { ClipboardWhere, ClipboardWithRelations } from "../types";
import { InvokeCommand } from "../types/tauri-invoke";
import { invokeCommand } from "../utils/tauri";

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

  const getClipboards = async () => {
    const response = await invokeCommand(InvokeCommand.GetClipboards, where());
    console.log(response);
    setHasMore(response.has_more);
    return response.clipboards;
  };

  const initClipboards = async () => {
    const clipboards = await getClipboards();

    setClipboards(clipboards);
  };

  const resetClipboards = async () => {
    setWhere(initialWhere);
    setHasMore(true);
    const clipboards = await getClipboards();
    setClipboards(clipboards);
  };

  return {
    clipboards,
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
    initClipboards,
  };
}

export const ClipboardStore = createRoot(createClipboardStore);
export type ClipboardStore = ReturnType<typeof createClipboardStore>;
