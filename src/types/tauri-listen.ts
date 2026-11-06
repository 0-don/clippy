import { ClipboardWithRelations, Progress, SyncStatus } from ".";
import { Tab } from "../utils/constants";
import { PasswordAction } from "./enums";

export enum ListenEvent {
  InitClipboards = "init_clipboards",
  InitSettings = "init_settings",
  InitHotkeys = "init_hotkeys",
  EnableGlobalHotkeyEvent = "enable_global_hotkey_event",
  ChangeTab = "change_tab",
  ScrollToTop = "scroll_to_top",
  NewClipboard = "new_clipboard",
  PasswordLock = "password_lock",
  Progress = "progress",
  SyncStatus = "sync_status",
}

export interface TauriListenEvents {
  [ListenEvent.InitClipboards]: void;
  [ListenEvent.InitSettings]: void;
  [ListenEvent.InitHotkeys]: void;
  [ListenEvent.EnableGlobalHotkeyEvent]: boolean;
  [ListenEvent.ChangeTab]: Tab;
  [ListenEvent.ScrollToTop]: void;
  [ListenEvent.NewClipboard]: ClipboardWithRelations;
  [ListenEvent.Progress]: Progress;
  [ListenEvent.PasswordLock]: PasswordAction;
  [ListenEvent.SyncStatus]: SyncStatus;
}
