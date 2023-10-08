import { IconTypes } from "solid-icons";

export const VIEW_MORE_NAMES = [
  "Sync Clipboard History",
  "Preferences",
  "About",
  "Exit",
] as const;

export type ViewMoreName = (typeof VIEW_MORE_NAMES)[number];

export const TAB_NAMES = [
  "Recent Clipboards",
  "Starred Clipboards",
  "History",
  "View more",
] as const;

export const TAB_IDS = [
  "recent_clipboards",
  "starred_clipboards",
  "history",
  "view_more",
] as const;

export type TabName = (typeof TAB_NAMES)[number];
export type TabId = (typeof TAB_IDS)[number];

export type Tabs = {
  name: TabName;
  Icon: IconTypes;
  current: boolean;
  id: TabId;
};

export const CLIPBOARD_HOTKEYS = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

export const GLOBAL_SHORTCUT_KEYS = [
  "none",
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z",
  "0",
  "1",
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
  "9",
  "F1",
  "F2",
  "F3",
  "F4",
  "F5",
  "F6",
  "F7",
  "F8",
  "F9",
  "F10",
  "F11",
  "F12",
] as const;

export type GlobalShortcutKeysType = (typeof GLOBAL_SHORTCUT_KEYS)[number];
