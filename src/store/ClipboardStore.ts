import { createRoot, createSignal } from "solid-js";
import { Clips } from "../@types";

type ClipboardWhere = {
  cursor?: number;
  search?: string;
  star?: boolean;
  show_images?: boolean;
};

const initialWhere: ClipboardWhere = {
  cursor: undefined,
  search: undefined,
  star: undefined,
  show_images: undefined,
};

function createClipboardStore() {
  const [clipboards, setClipboards] = createSignal<Clips[]>([]);
  const [where, setWhere] = createSignal<ClipboardWhere>(initialWhere);

  const resetWhere = () => setWhere(initialWhere);

  return {
    clipboards,
    setClipboards,
    where,
    setWhere,
    resetWhere,
  };
}

export default createRoot(createClipboardStore);
