import { ClipboardWithRelations } from ".";
import { Tab } from "../utils/constants";

export enum ListenEvent {
  Init = "init",
  EnableGlobalHotkeyEvent = "enable_global_hotkey_event",
  ChangeTab = "change_tab",
  ScrollToTop = "scroll_to_top",
  NewClipboard = "new_clipboard",
}

export interface TauriListenEvents {
  [ListenEvent.Init]: void;
  [ListenEvent.EnableGlobalHotkeyEvent]: boolean;
  [ListenEvent.ChangeTab]: Tab;
  [ListenEvent.ScrollToTop]: void;
  [ListenEvent.NewClipboard]: ClipboardWithRelations;
}
