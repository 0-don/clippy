import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { VsClose } from "solid-icons/vs";
import { Component, createSignal, onCleanup, Show } from "solid-js";
import { DictionaryKey } from "../../lib/i18n";
import { invokeCommand } from "../../lib/tauri";
import { cn } from "../../lib/utils";
import { AppStore } from "../../store/app-store";
import { InvokeCommand } from "../../types/tauri-invoke";
import {
  GLOBAL_SHORTCUT_KEYS,
  GlobalShortcutKeys,
} from "../../utils/constants";
import { useLanguage } from "../provider/language-provider";

export type RecordedHotkey = {
  ctrl: boolean;
  alt: boolean;
  shift: boolean;
  super_key: boolean;
  key: GlobalShortcutKeys;
};

const MODIFIER_CODES = [
  "ControlLeft",
  "ControlRight",
  "AltLeft",
  "AltRight",
  "AltGraph",
  "ShiftLeft",
  "ShiftRight",
  "MetaLeft",
  "MetaRight",
  "OSLeft",
  "OSRight",
];

const PUNCTUATION: Record<string, GlobalShortcutKeys> = {
  "-": "Minus",
  "=": "Equal",
  "[": "BracketLeft",
  "]": "BracketRight",
  "\\": "Backslash",
  ";": "Semicolon",
  "'": "Quote",
  ",": "Comma",
  ".": "Period",
  "/": "Slash",
  "`": "Backquote",
};

// The key list uses KeyboardEvent.code names, except letters and digits.
const keyFromCode = (code: string) => {
  const name = code.replace(/^Key([A-Z])$/, "$1").replace(/^Digit(\d)$/, "$1");
  return GLOBAL_SHORTCUT_KEYS.find((key) => key === name);
};

// Linux and Windows register hotkeys by the character the layout produces, so
// the Y key on QWERTZ must record as Y, not as the US position Z. macOS
// registers physical positions. The layout map comes from the OS and ignores
// Shift and AltGr; e.key covers setups where it is empty. Digits stay
// positional (AZERTY types them shifted), and anything without a usable
// character falls back to the physical key.
const keyFromLayout = (code: string, layout: Record<string, string>) => {
  if (code.startsWith("Digit")) return undefined;
  const char = layout[code]?.toUpperCase();
  if (!char) return undefined;
  if (/^[A-Z]$/.test(char)) return keyFromCode(`Key${char}`);
  return PUNCTUATION[char];
};

const keyFromEvent = (
  e: KeyboardEvent,
  os: string | undefined,
  layout: Record<string, string>,
) => {
  if (os === "macos") return keyFromCode(e.code);

  const fromLayout = keyFromLayout(e.code, layout);
  if (fromLayout) return fromLayout;

  if (e.key.length === 1) {
    const char = e.key.toUpperCase();
    if (/^[A-Z]$/.test(char)) return keyFromCode(`Key${char}`);
    if (!e.shiftKey && PUNCTUATION[char]) return PUNCTUATION[char];
  }
  return keyFromCode(e.code);
};

const SUPER_LABELS: Record<string, DictionaryKey> = {
  macos: "MAIN.KEYS.SUPER_MAC",
  windows: "MAIN.KEYS.SUPER_WIN",
};

interface HotkeyRecorderProps {
  value: Omit<RecordedHotkey, "key"> & { key: string };
  icon?: boolean;
  onRecord: (hotkey: RecordedHotkey) => void;
  onClear: () => void;
}

export const HotkeyRecorder: Component<HotkeyRecorderProps> = (props) => {
  const { t } = useLanguage();
  const [recording, setRecording] = createSignal(false);

  const isSet = () => props.value.key !== "none" && props.value.key !== "";

  const label = () =>
    [
      props.value.ctrl && t("MAIN.KEYS.CTRL"),
      props.value.alt && t("MAIN.KEYS.ALT"),
      props.value.shift && t("MAIN.KEYS.SHIFT"),
      props.value.super_key &&
        t(SUPER_LABELS[AppStore.os() ?? ""] ?? "MAIN.KEYS.SUPER_LINUX"),
      props.value.key,
    ]
      .filter(Boolean)
      .join(" + ");

  const onKeyDown = (e: KeyboardEvent) => {
    e.preventDefault();
    e.stopImmediatePropagation();
    if (MODIFIER_CODES.includes(e.code)) return;

    const modified = e.ctrlKey || e.altKey || e.shiftKey || e.metaKey;
    if (e.code === "Escape" && !modified) return stop();

    const key = keyFromEvent(e, AppStore.os(), layout);
    if (!key) return;

    props.onRecord({
      ctrl: e.ctrlKey,
      alt: e.altKey,
      shift: e.shiftKey,
      super_key: e.metaKey,
      key,
    });
    stop();
  };

  let layout: Record<string, string> = {};

  // Read on every start, the user may have switched layouts since.
  const start = () => {
    setRecording(true);
    invokeCommand(InvokeCommand.GetKeyboardLayout)
      .then((map) => (layout = map))
      .catch(() => (layout = {}));
    window.addEventListener("keydown", onKeyDown, true);
    window.addEventListener("blur", stop);
  };

  function stop() {
    setRecording(false);
    window.removeEventListener("keydown", onKeyDown, true);
    window.removeEventListener("blur", stop);
  }

  onCleanup(stop);

  return (
    <div class="flex items-center gap-1">
      <button
        type="button"
        class={cn(
          "bg-popover flex min-w-44 cursor-pointer items-center gap-2 rounded-md border px-2 py-1 text-left text-sm",
          recording()
            ? "border-primary text-primary"
            : "border-border hover:border-primary",
        )}
        title={t("MAIN.KEYS.RECORD")}
        onClick={() => (recording() ? stop() : start())}
      >
        <Show when={props.icon !== false}>
          <RiDeviceKeyboardFill
            class={cn("shrink-0", recording() && "animate-pulse")}
          />
        </Show>
        <span
          class={cn(
            "truncate",
            !recording() && !isSet() && "text-muted-foreground",
          )}
        >
          {recording()
            ? t("MAIN.KEYS.PRESS_KEYS")
            : isSet()
              ? label()
              : t("MAIN.KEYS.RECORD")}
        </span>
      </button>
      <button
        type="button"
        class={cn(
          "text-muted-foreground hover:text-foreground cursor-pointer p-1",
          (!isSet() || recording()) && "invisible",
        )}
        onClick={() => props.onClear()}
      >
        <VsClose />
      </button>
    </div>
  );
};
