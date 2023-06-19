import { invoke } from "@tauri-apps/api";
import { createRoot, createSignal } from "solid-js";
import { Clips } from "../@types";

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
  const [clipboards, setClipboards] = createSignal<Clips[]>([]);
  const [where, setWhere] = createSignal<ClipboardWhere>(initialWhere);

  const resetWhere = () => setWhere(initialWhere);

  const getClipboards = async () =>
    await invoke<Clips[]>("get_clipboards", where());

  return {
    clipboards,
    setClipboards,
    where,
    setWhere,
    resetWhere,
    getClipboards,
  };
}

export default createRoot(createClipboardStore);
