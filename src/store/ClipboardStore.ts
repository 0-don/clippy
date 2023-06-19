import { invoke } from "@tauri-apps/api";
import { createRoot, createSignal } from "solid-js";
import { Clips } from "../@types";

type ClipboardWhere = {
  cursor?: number;
  search?: string;
  star?: boolean;
  show_images?: boolean;
};

export const initialWhere: ClipboardWhere = {
  cursor: undefined,
  search: undefined,
  star: undefined,
  show_images: undefined,
};

function createClipboardStore() {
  const [clipboards, setClipboards] = createSignal<Clips[]>([]);
  const [where, setWhere] = createSignal<ClipboardWhere>(initialWhere);

  const resetWhere = () => setWhere(initialWhere);

  async function getClipboards() {
    const params = where();
    const newClipboards = await invoke<Clips[]>(
      "infinite_scroll_clipboards",
      params
    );
    return newClipboards;
  }

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
