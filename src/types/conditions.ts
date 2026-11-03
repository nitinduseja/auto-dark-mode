export interface ConditionsConfig {
  pause_on_battery: boolean;
  force_dark_on_battery_saver: boolean;
  pause_on_gaming: boolean;
  idle_delay_minutes: number;
  pause_on_metered: boolean;
}

export interface PowerStatus {
  is_on_battery: boolean;
  battery_percent: number;
  is_charging: boolean;
  is_battery_saver_on: boolean;
}

export interface SystemConditionsStatus {
  power: PowerStatus;
  is_gaming_active: boolean;
  is_switch_allowed: boolean;
  restriction_reason: string | null;
}
