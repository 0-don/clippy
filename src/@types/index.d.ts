import { SidebarIconName, ViewMoreName } from '../store/utils/constants'

export type Clips = {
  id?: number | null
  type: ClipboardType
  content?: string | null
  width?: number | null
  height?: number | null
  size?: string | null
  blob?: number[]
  base64?: string
  star?: boolean
  created_date?: Date
}

export type ClipboardType = 'text' | 'image' | 'hex' | 'rgb' | 'link'

export type Hotkey = {
  id: number
  event: HotkeyEvent
  ctrl: boolean
  alt: boolean
  shift: boolean
  key: string
  status: boolean
  name: ViewMoreName & SidebarIconName
  icon: string

  shortcut: string // not in db added for convenience
}

export type Settings = {
  id: number
  startup: boolean
  notification: boolean
  synchronize: boolean
  dark_mode: boolean
}

export type HotkeyEvent =
  | 'window_display_toggle'
  | 'type_clipboard'
  | 'recent_clipboards'
  | 'recent_clipboards'
  | 'history'
  | 'view_more'
  | 'sync_clipboard_history'
  | 'preferences'
  | 'about'
  | 'exit'
  | 'toggle_dev_tools'
  | 'scroll_to_top'

export type WindowName = 'about' | 'settings'
