export type ScheduleType = 'sunrise_sunset' | 'custom_times' | 'night_light';

export interface ScheduleConfig {
  type: ScheduleType;
  enabled: boolean;
  light_start_hour: number;
  light_start_minute: number;
  dark_start_hour: number;
  dark_start_minute: number;
  latitude: number;
  longitude: number;
  location_name: string;
  use_location_service: boolean;
  sunrise_offset_minutes: number;
  sunset_offset_minutes: number;
  idle_delay_minutes: number;
  pause_on_battery: boolean;
  pause_on_gaming: boolean;
}

export interface SolarPreviewRequest {
  latitude: number;
  longitude: number;
  sunrise_offset_minutes: number;
  sunset_offset_minutes: number;
}

export interface SolarPreviewResult {
  sunrise_minute: number;
  sunset_minute: number;
  sunrise_time_formatted: string;
  sunset_time_formatted: string;
  daylight_window_formatted: string;
}

export interface CityPreset {
  name: string;
  latitude: number;
  longitude: number;
}

export const POPULAR_CITIES: CityPreset[] = [
  { name: 'New York, USA', latitude: 40.7128, longitude: -74.006 },
  { name: 'Los Angeles, USA', latitude: 34.0522, longitude: -118.2437 },
  { name: 'London, UK', latitude: 51.5074, longitude: -0.1278 },
  { name: 'Berlin, Germany', latitude: 52.52, longitude: 13.405 },
  { name: 'Paris, France', latitude: 48.8566, longitude: 2.3522 },
  { name: 'Tokyo, Japan', latitude: 35.6762, longitude: 139.6503 },
  { name: 'Sydney, Australia', latitude: -33.8688, longitude: 151.2093 },
  { name: 'Seattle, USA', latitude: 47.6062, longitude: -122.3321 },
  { name: 'Toronto, Canada', latitude: 43.6532, longitude: -79.3832 },
  { name: 'Singapore', latitude: 1.3521, longitude: 103.8198 },
];
