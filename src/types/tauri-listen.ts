import { Tab } from "../utils/constants";

export enum ListenEvent {
  Init = "init",
  SetGlobalHotkeyEvent = "set_global_hotkey_event",
  ChangeTab = "change_tab",
}

export interface TauriListenEvents {
  [ListenEvent.Init]: void;
  [ListenEvent.SetGlobalHotkeyEvent]: boolean;
  [ListenEvent.ChangeTab]: Tab;
}
