import { invoke } from "@tauri-apps/api/core";
import { createRoot, createSignal } from "solid-js";
import { ClipboardWhere, ClipboardWithRelations } from "../types";
import { invokeCommand } from "../utils/tauri";
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

  const resetWhere = () => setWhere(initialWhere);

  const getClipboards = async () => {
    const clipboards = await invokeCommand(InvokeCommand.GetClipboards, where());
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
