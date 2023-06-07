import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { createEffect, onCleanup } from "solid-js";
import {
  IMAGE_CHANGED,
  TEXT_CHANGED,
  listenImage,
  listenText,
} from "tauri-plugin-clipboard-api";

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
    tauriTextUnlisten = await listen(TEXT_CHANGED, ({ payload }: Listener) => {
      console.log(payload.value);
    });

    tauriImageUnlisten = await listen(
      IMAGE_CHANGED,
      ({ payload }: Listener) => {
        console.log(payload.value);
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
