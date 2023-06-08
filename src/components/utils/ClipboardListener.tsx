import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { createEffect, onCleanup } from "solid-js";
import {
  IMAGE_CHANGED,
  TEXT_CHANGED,
  listenImage,
  listenText,
} from "tauri-plugin-clipboard-api";
import { Clipboards } from "../../@types";
import { formatBytes } from "../../utils/helpers";

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
    console.log("wtf");
    // tauriTextUnlisten = await listen(TEXT_CHANGED, async (event: Listener) => {
    //   console.log(event);
    //   const clipboard: Clipboards = {
    //     id: 0,
    //     type: "text",
    //     content: event.payload.value,
    //   };

    //   console.log(clipboard);
    //   // await invoke("insert_clipboard", { clipboard });
    // });

    // tauriImageUnlisten = await listen(
    //   IMAGE_CHANGED,
    //   async (event: Listener) => {
    //     console.log(event);
    //     const base64 = event.payload.value;
    //     const img = new Image();

    //     const blob = new Uint8Array(
    //       atob(base64)
    //         .split("")
    //         .map((char) => char.charCodeAt(0))
    //     );

    //     // promisfy image load

    //     const clipboard = await new Promise<Clipboards>((resolve, reject) => {
    //       img.onload = function () {
    //         const i = this as HTMLImageElement;
    //         const width = i.naturalWidth;
    //         const height = i.naturalHeight;
    //         const size = blob.length;

    //         const clipboard: Clipboards = {
    //           id: 0,
    //           type: "image",
    //           width,
    //           height,
    //           size: formatBytes(size),
    //           blob,
    //         };
    //         resolve(clipboard);
    //       };
    //       img.src = `data:image/png;base64,${base64}`;
    //     });

    //     console.log(clipboard);

    //     // await invoke("insert_clipboard", { clipboard });
    //   }
    // );

    imageUnlisten = listenImage();
    textUnlisten = listenText();

    onCleanup(() => {
      imageUnlisten();
      textUnlisten();
      // tauriTextUnlisten();
      // tauriImageUnlisten();
    });
  });
  return true;
};
