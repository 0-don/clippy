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
  const [clipboardRef, setClipboardRef] = createSignal<
    HTMLDivElement | undefined
  >();
  const [clipboards, setClipboards] = createSignal<Clips[]>([]);
  const [where, setWhere] = createSignal<ClipboardWhere>(initialWhere);

  const resetWhere = () => setWhere(initialWhere);

  const getClipboards = async () => {
    const clipboards = await invoke<Clips[]>("get_clipboards", where());
    const newClipboards = await Promise.all(
      clipboards.map(async (clipboard) => {
        if (clipboard.type === "image") {
          const base64data = await uint8ArrayToBase64(
            clipboard.blob as number[],
          );
          return {
            ...clipboard,
            blob: base64data,
          };
        }
        return clipboard;
      }),
    );
    return newClipboards;
  };

  function uint8ArrayToBase64(byteArray: number[]): Promise<string> {
    return new Promise((resolve, reject) => {
      const blob = new Blob([new Uint8Array(byteArray)], { type: "image/png" });
      const reader = new FileReader();
      reader.onloadend = () => resolve(reader.result as string);
      reader.onerror = (error) => reject(error);
      reader.readAsDataURL(blob);
    });
  }
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
