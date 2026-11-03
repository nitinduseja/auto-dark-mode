use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerStatus {
    pub is_on_battery: bool,
    pub battery_percent: u8,
    pub is_charging: bool,
    pub is_battery_saver_on: bool,
}

impl Default for PowerStatus {
    fn default() -> Self {
        Self {
            is_on_battery: false,
            battery_percent: 100,
            is_charging: true,
            is_battery_saver_on: false,
        }
    }
}

pub fn get_power_status() -> PowerStatus {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

        let mut status = SYSTEM_POWER_STATUS::default();
        let success = unsafe { GetSystemPowerStatus(&mut status) };

        if success.is_ok() {
            let is_on_battery = status.ACLineStatus == 0;
            let is_charging = status.ACLineStatus == 1 || (status.BatteryFlag & 8) != 0;
            let battery_percent = if status.BatteryLifePercent <= 100 {
                status.BatteryLifePercent
            } else {
                100
            };
            let is_battery_saver_on = status.SystemStatusFlag == 1;

            PowerStatus {
                is_on_battery,
                battery_percent,
                is_charging,
                is_battery_saver_on,
            }
        } else {
            PowerStatus::default()
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        PowerStatus::default()
    }
}
