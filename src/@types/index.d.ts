import { SidebarIconName, ViewMoreName } from "../store/utils/constants";

export type ClipboardType = "text" | "image" | "html" | "rtf" | "file";

export type ClipboardTextType = "text" | "link" | "hex" | "rgb";

export type DateTime = string;

export interface ClipboardModel {
  id: number;
  types: ClipboardType[];
  star: boolean;
  created_date: DateTime;
}

export interface ClipboardTextModel {
  id: number;
  clipboard_id: number;
  type: ClipboardTextType;
  data: string;
}

export interface ClipboardHtmlModel {
  id: number;
  clipboard_id: number;
  data: string;
}

export interface ClipboardImageModel {
  id: number;
  clipboard_id: number;
  data: Uint8Array;
  extension: string | null;
  width: number | null;
  height: number | null;
  size: string | null;
  thumbnail: string | null;
}

export interface ClipboardRtfModel {
  id: number;
  clipboard_id: number;
  data: string;
}

export interface ClipboardFileModel {
  id: number;
  clipboard_id: number;
  data: Uint8Array;
  name: string | null;
  extension: string | null;
  size: number | null;
  created_date: DateTime | null;
  updated_date: DateTime | null;
}

export interface ClipboardWithRelations {
  clipboard: ClipboardModel;
  text?: ClipboardTextModel;
  html?: ClipboardHtmlModel;
  image?: ClipboardImageModel;
  rtf?: ClipboardRtfModel;
  file?: ClipboardFileModel;
}

export type Hotkey = {
  id: number;
  event: HotkeyEvent;
  ctrl: boolean;
  alt: boolean;
  shift: boolean;
  key: string;
  status: boolean;
  name: ViewMoreName & SidebarIconName;
  icon: string;

  shortcut: string; // not in db added for convenience
};

export type Settings = {
  id: number;
  startup: boolean;
  notification: boolean;
  synchronize: boolean;
  dark_mode: boolean;
};

export type HotkeyEvent =
  | "window_display_toggle"
  | "type_clipboard"
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

export type WindowName = "About" | "Settings";
