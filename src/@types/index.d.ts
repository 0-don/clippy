export type Clipboards = {
  id?: number | null;
  type: ClipboardType;
  content?: string | null;
  width?: number | null;
  height?: number | null;
  size?: string | null;
  blob?: Uint8Array | null;
  star?: boolean;
  created_date?: Date;
};

export type ClipboardType = "text" | "image" | "color";

export type Hotkey = {
  id: number;
  event: HotkeyEvent;
  ctrl: boolean;
  alt: boolean;
  shift: boolean;
  key: string;
  status: boolean;
  name: string;
  icon: string;

  shortcut: string; // not in db added for convenience
};

export type HotkeyEvent =
  | "window_display_toggle"
  | "recent_clipboards"
  | "recent_clipboards"
  | "history"
  | "view_more"
  | "sync_clipboard_history"
  | "preferences"
  | "about"
  | "exit"
  | "toggle_dev_tools"
  | "scroll_to_top";

export type Settings = {
  id: number;
  startup: boolean;
  notification: boolean;
  synchronize: boolean;
  synchronize_time: number;
  dar_kmode: boolean;
};
