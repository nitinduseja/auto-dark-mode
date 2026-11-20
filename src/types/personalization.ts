export type WallpaperFit = 'fill' | 'fit' | 'stretch' | 'tile' | 'center' | 'span';

export interface ConnectedMonitor {
  id: string;
  index: number;
  name: string;
  width: number;
  height: number;
  is_primary: boolean;
}

export interface MonitorWallpaperConfig {
  monitor_id: string;
  monitor_name: string;
  light_wallpaper_path: string;
  dark_wallpaper_path: string;
}

export interface PersonalizationConfig {
  wallpaper_enabled: boolean;
  multi_monitor_enabled: boolean;
  light_wallpaper_path: string;
  dark_wallpaper_path: string;
  monitor_wallpapers: MonitorWallpaperConfig[];
  wallpaper_fit: WallpaperFit;
  sync_accent_color: boolean;
  light_accent_color: string;
  dark_accent_color: string;
  sync_cursor: boolean;
  sync_sounds: boolean;
  sync_color_filter: boolean;
  sync_touch_keyboard: boolean;
  colorize_taskbar: boolean;
  colorize_titlebars: boolean;
}
