import { IconTypes } from "solid-icons";
import { SettingsTabName, Tab, TabName } from "../utils/constants";
import { ClipboardTextType, ClipboardType, HotkeyEvent } from "./enums";

export type DatabaseInfo = {
  records: number;
  size: number;
};

export type Tabs = {
  name: TabName;
  Icon: IconTypes;
  current: boolean;
  id: Tab;
};

export type ClipboardWhere = {
  cursor?: number;
  search?: string;
  star?: boolean;
  img?: boolean;
};

export type SettingsTab = {
  name: SettingsTabName;
  Icon: IconTypes;
  current: boolean;
};

export interface ClipboardFileModel {
  id: number;
  clipboard_id: number;
  data: Uint8Array;
  name: string | null;
  extension: string | null;
  size: number | null;
  mime_type: string | null;
  created_date: string | null;
  updated_date: string | null;
}

export interface ClipboardModel {
  id: number;
  types: ClipboardType[];
  star: boolean;
  created_date: string;
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

export interface ClipboardWithRelations {
  clipboard: ClipboardModel;
  text?: ClipboardTextModel;
  html?: ClipboardHtmlModel;
  image?: ClipboardImageModel;
  rtf?: ClipboardRtfModel;
  files?: ClipboardFileModel[];
}

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

export type Settings = {
  id: number;
  startup: boolean;
  notification: boolean;
  synchronize: boolean;
  dark_mode: boolean;
};
