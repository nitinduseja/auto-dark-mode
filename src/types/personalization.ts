export type WallpaperFit = 'fill' | 'fit' | 'stretch' | 'tile' | 'center' | 'span';

export interface PersonalizationConfig {
  wallpaper_enabled: boolean;
  light_wallpaper_path: string;
  dark_wallpaper_path: string;
  wallpaper_fit: WallpaperFit;
  sync_accent_color: boolean;
  light_accent_color: string;
  dark_accent_color: string;
  sync_cursor: boolean;
  sync_sounds: boolean;
  sync_color_filter: boolean;
  sync_touch_keyboard: boolean;
}
