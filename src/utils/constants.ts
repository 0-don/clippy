import { HotkeyEvent } from "../types/enums";

export const SETTINGS_TAB = ["General", "Backup", "History", "Hotkeys"] as const;
export const VIEW_MORE_NAMES = ["Sync Clipboard History", "Settings", "About", "Exit"] as const;
export const TAB_NAMES = ["Recent Clipboards", "Starred Clipboards", "History", "View more"] as const;
export const TABS = [
  HotkeyEvent.RecentClipboards,
  HotkeyEvent.StarredClipboards,
  HotkeyEvent.History,
  HotkeyEvent.ViewMore,
] as const;

export type SettingsTabName = (typeof SETTINGS_TAB)[number];
export type ViewMoreName = (typeof VIEW_MORE_NAMES)[number];
export type TabName = (typeof TAB_NAMES)[number];
export type Tab = (typeof TABS)[number];

export type GlobalShortcutKeys = (typeof GLOBAL_SHORTCUT_KEYS)[number];
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
