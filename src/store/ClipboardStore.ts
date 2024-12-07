import { invoke } from "@tauri-apps/api/core";
import { createRoot, createSignal } from "solid-js";
import { ClipboardWithRelations } from "../@types";

type ClipboardWhere = {
  cursor?: number;
  search?: string;
  star?: boolean;
  img?: boolean;
};

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

  const resetWhere = () => setWhere(initialWhere);

  const getClipboards = async () => {
    const clipboards = await invoke<ClipboardWithRelations[]>("get_clipboards", where());
    return clipboards;
  };

  const initClipboards = async () => {
    const clipboards = await getClipboards();
    console.log("clipboards", clipboards);
    setClipboards(clipboards);
  };

  return {
    clipboards,
    setClipboards,
    where,
    setWhere,
    resetWhere,
    getClipboards,
    clipboardRef,
    setClipboardRef,
    initClipboards,
  };
}

export default createRoot(createClipboardStore);
