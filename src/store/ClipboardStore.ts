import { createRoot, createSignal } from "solid-js";
import { Clips } from "../@types";
import { invoke } from "@tauri-apps/api/core";

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
  const [clipboards, setClipboards] = createSignal<Clips[]>([]);
  const [where, setWhere] = createSignal<ClipboardWhere>(initialWhere);

  const resetWhere = () => setWhere(initialWhere);

  const getClipboards = async () => {
    const clipboards = await invoke<Clips[]>("get_clipboards", where());
    return clipboards;
  };

  const initClipboards = async () => {
    const clipboards = await getClipboards();
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
