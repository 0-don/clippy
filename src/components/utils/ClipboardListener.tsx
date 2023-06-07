import { invoke } from "@tauri-apps/api";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { createEffect, onCleanup } from "solid-js";
import {
  IMAGE_CHANGED,
  TEXT_CHANGED,
  listenImage,
  listenText,
} from "tauri-plugin-clipboard-api";
import { Clipboards } from "../../@types";

let tauriTextUnlisten: UnlistenFn;
let tauriImageUnlisten: UnlistenFn;
let textUnlisten: () => void;
let imageUnlisten: () => void;

type Listener = {
  payload: {
    value: string;
  };
};

export const ClipboardListener = () => {
  createEffect(async () => {
    tauriTextUnlisten = await listen(
      TEXT_CHANGED,
      async ({ payload }: Listener) => {
        const clipboard: Clipboards = {
          type: "text",
          content: payload.value,
        };
        await invoke("insert_clipboard", { clipboard });
      }
    );

    tauriImageUnlisten = await listen(
      IMAGE_CHANGED,
      ({ payload }: Listener) => {
        const base64 = payload.value;
        const img = new Image();

        const blob = new Uint8Array(
          atob(base64)
            .split("")
            .map((char) => char.charCodeAt(0))
        );
        img.onload = async function () {
          const i = this as HTMLImageElement;
          const width = i.naturalWidth;
          const height = i.naturalHeight;
          const size = blob.length;
        };
        img.src = `data:image/png;base64,${base64}`;
      }
    );

    imageUnlisten = listenImage();
    textUnlisten = listenText();

    onCleanup(() => {
      imageUnlisten();
      textUnlisten();
      tauriTextUnlisten();
      tauriImageUnlisten();
    });
  });
  return true;
};
