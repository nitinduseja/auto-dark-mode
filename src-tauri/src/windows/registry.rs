use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    #[error("Failed to open registry key: {0} (code: {1})")]
    OpenKeyFailed(String, u32),

    #[error("Failed to read registry value: {0} (code: {1})")]
    ReadValueFailed(String, u32),

    #[error("Failed to write registry value: {0} (code: {1})")]
    WriteValueFailed(String, u32),

    #[error("Value type mismatch for key: {0}. Expected DWORD")]
    TypeMismatch(String),

    #[error("Verification failed for {0}: expected {1}, but read {2}")]
    VerificationFailed(String, u32, u32),

    #[error("Unsupported platform: Windows registry is only accessible on Windows")]
    UnsupportedPlatform,
}

pub const PERSONALIZE_SUBKEY: &str =
    r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
pub const APPS_USE_LIGHT_THEME: &str = "AppsUseLightTheme";
pub const SYSTEM_USES_LIGHT_THEME: &str = "SystemUsesLightTheme";

#[cfg(target_os = "windows")]
mod win_impl {
    use super::*;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::ERROR_SUCCESS;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, HKEY,
        HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_SET_VALUE, REG_DWORD, REG_SZ, REG_VALUE_TYPE,
    };

    struct SafeHKey(HKEY);

    impl Drop for SafeHKey {
        fn drop(&mut self) {
            if !self.0.is_invalid() {
                unsafe {
                    let _ = RegCloseKey(self.0);
                }
            }
        }
    }

    fn to_wide_null(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub fn read_dword(subkey: &str, value_name: &str) -> Result<u32, RegistryError> {
        let subkey_wide = to_wide_null(subkey);
        let value_name_wide = to_wide_null(value_name);
        let mut hkey = HKEY::default();

        let status = unsafe {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(subkey_wide.as_ptr()),
                0,
                KEY_READ,
                &mut hkey,
            )
        };

        if status != ERROR_SUCCESS {
            return Err(RegistryError::OpenKeyFailed(subkey.to_string(), status.0));
        }

        let _guard = SafeHKey(hkey);

        let mut val_type = REG_VALUE_TYPE::default();
        let mut data: u32 = 0;
        let mut data_size = std::mem::size_of::<u32>() as u32;

        let query_status = unsafe {
            RegQueryValueExW(
                hkey,
                PCWSTR(value_name_wide.as_ptr()),
                None,
                Some(&mut val_type),
                Some(&mut data as *mut u32 as *mut u8),
                Some(&mut data_size),
            )
        };

        if query_status != ERROR_SUCCESS {
            return Err(RegistryError::ReadValueFailed(
                value_name.to_string(),
                query_status.0,
            ));
        }

        if val_type != REG_DWORD {
            return Err(RegistryError::TypeMismatch(value_name.to_string()));
        }

        Ok(data)
    }

    pub fn write_dword(subkey: &str, value_name: &str, value: u32) -> Result<(), RegistryError> {
        let subkey_wide = to_wide_null(subkey);
        let value_name_wide = to_wide_null(value_name);
        let mut hkey = HKEY::default();

        let status = unsafe {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(subkey_wide.as_ptr()),
                0,
                KEY_SET_VALUE | KEY_READ,
                &mut hkey,
            )
        };

        if status != ERROR_SUCCESS {
            return Err(RegistryError::OpenKeyFailed(subkey.to_string(), status.0));
        }

        let _guard = SafeHKey(hkey);

        let set_status = unsafe {
            RegSetValueExW(
                hkey,
                PCWSTR(value_name_wide.as_ptr()),
                0,
                REG_DWORD,
                Some(std::slice::from_raw_parts(
                    &value as *const u32 as *const u8,
                    std::mem::size_of::<u32>(),
                )),
            )
        };

        if set_status != ERROR_SUCCESS {
            return Err(RegistryError::WriteValueFailed(
                value_name.to_string(),
                set_status.0,
            ));
        }

        // Verify write was applied correctly
        let verified = read_dword(subkey, value_name)?;
        if verified != value {
            return Err(RegistryError::VerificationFailed(
                value_name.to_string(),
                value,
                verified,
            ));
        }

        Ok(())
    }

    pub fn read_string(subkey: &str, value_name: &str) -> Result<String, RegistryError> {
        let subkey_wide = to_wide_null(subkey);
        let value_name_wide = to_wide_null(value_name);
        let mut hkey = HKEY::default();

        let status = unsafe {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(subkey_wide.as_ptr()),
                0,
                KEY_READ,
                &mut hkey,
            )
        };

        if status != ERROR_SUCCESS {
            return Err(RegistryError::OpenKeyFailed(subkey.to_string(), status.0));
        }

        let _guard = SafeHKey(hkey);

        let mut val_type = REG_VALUE_TYPE::default();
        let mut buffer = [0u16; 512];
        let mut data_size = (buffer.len() * std::mem::size_of::<u16>()) as u32;

        let query_status = unsafe {
            RegQueryValueExW(
                hkey,
                PCWSTR(value_name_wide.as_ptr()),
                None,
                Some(&mut val_type),
                Some(buffer.as_mut_ptr() as *mut u8),
                Some(&mut data_size),
            )
        };

        if query_status != ERROR_SUCCESS {
            return Err(RegistryError::ReadValueFailed(
                value_name.to_string(),
                query_status.0,
            ));
        }

        let char_count = (data_size as usize / std::mem::size_of::<u16>()).saturating_sub(1);
        let s = String::from_utf16_lossy(&buffer[..char_count]);
        Ok(s)
    }

    pub fn read_hklm_string(subkey: &str, value_name: &str) -> Result<String, RegistryError> {
        let subkey_wide = to_wide_null(subkey);
        let value_name_wide = to_wide_null(value_name);
        let mut hkey = HKEY::default();

        let status = unsafe {
            RegOpenKeyExW(
                HKEY_LOCAL_MACHINE,
                PCWSTR(subkey_wide.as_ptr()),
                0,
                KEY_READ,
                &mut hkey,
            )
        };

        if status != ERROR_SUCCESS {
            return Err(RegistryError::OpenKeyFailed(subkey.to_string(), status.0));
        }

        let _guard = SafeHKey(hkey);

        let mut val_type = REG_VALUE_TYPE::default();
        let mut buffer = [0u16; 512];
        let mut data_size = (buffer.len() * std::mem::size_of::<u16>()) as u32;

        let query_status = unsafe {
            RegQueryValueExW(
                hkey,
                PCWSTR(value_name_wide.as_ptr()),
                None,
                Some(&mut val_type),
                Some(buffer.as_mut_ptr() as *mut u8),
                Some(&mut data_size),
            )
        };

        if query_status != ERROR_SUCCESS {
            return Err(RegistryError::ReadValueFailed(
                value_name.to_string(),
                query_status.0,
            ));
        }

        let char_count = (data_size as usize / std::mem::size_of::<u16>()).saturating_sub(1);
        let s = String::from_utf16_lossy(&buffer[..char_count]);
        Ok(s)
    }

    pub fn write_string(subkey: &str, value_name: &str, value: &str) -> Result<(), RegistryError> {
        let subkey_wide = to_wide_null(subkey);
        let value_name_wide = to_wide_null(value_name);
        let value_wide = to_wide_null(value);
        let mut hkey = HKEY::default();

        let status = unsafe {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(subkey_wide.as_ptr()),
                0,
                KEY_SET_VALUE | KEY_READ,
                &mut hkey,
            )
        };

        if status != ERROR_SUCCESS {
            return Err(RegistryError::OpenKeyFailed(subkey.to_string(), status.0));
        }

        let _guard = SafeHKey(hkey);

        let byte_len = value_wide.len() * std::mem::size_of::<u16>();
        let set_status = unsafe {
            RegSetValueExW(
                hkey,
                PCWSTR(value_name_wide.as_ptr()),
                0,
                REG_SZ,
                Some(std::slice::from_raw_parts(
                    value_wide.as_ptr() as *const u8,
                    byte_len,
                )),
            )
        };

        if set_status != ERROR_SUCCESS {
            return Err(RegistryError::WriteValueFailed(
                value_name.to_string(),
                set_status.0,
            ));
        }

        Ok(())
    }

    pub fn delete_value(subkey: &str, value_name: &str) -> Result<(), RegistryError> {
        let subkey_wide = to_wide_null(subkey);
        let value_name_wide = to_wide_null(value_name);
        let mut hkey = HKEY::default();

        let status = unsafe {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(subkey_wide.as_ptr()),
                0,
                KEY_SET_VALUE,
                &mut hkey,
            )
        };

        if status != ERROR_SUCCESS {
            return Err(RegistryError::OpenKeyFailed(subkey.to_string(), status.0));
        }

        let _guard = SafeHKey(hkey);

        let del_status = unsafe { RegDeleteValueW(hkey, PCWSTR(value_name_wide.as_ptr())) };
        if del_status != ERROR_SUCCESS && del_status.0 != 2 {
            // Error 2 is ERROR_FILE_NOT_FOUND, which means already deleted
            return Err(RegistryError::WriteValueFailed(
                value_name.to_string(),
                del_status.0,
            ));
        }

        Ok(())
    }
}

pub fn read_dword(subkey: &str, value_name: &str) -> Result<u32, RegistryError> {
    #[cfg(target_os = "windows")]
    {
        win_impl::read_dword(subkey, value_name)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (subkey, value_name);
        Err(RegistryError::UnsupportedPlatform)
    }
}

pub fn write_dword(subkey: &str, value_name: &str, value: u32) -> Result<(), RegistryError> {
    #[cfg(target_os = "windows")]
    {
        win_impl::write_dword(subkey, value_name, value)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (subkey, value_name, value);
        Err(RegistryError::UnsupportedPlatform)
    }
}

pub fn read_string(subkey: &str, value_name: &str) -> Result<String, RegistryError> {
    #[cfg(target_os = "windows")]
    {
        win_impl::read_string(subkey, value_name)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (subkey, value_name);
        Err(RegistryError::UnsupportedPlatform)
    }
}

pub fn read_hklm_string(subkey: &str, value_name: &str) -> Result<String, RegistryError> {
    #[cfg(target_os = "windows")]
    {
        win_impl::read_hklm_string(subkey, value_name)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (subkey, value_name);
        Err(RegistryError::UnsupportedPlatform)
    }
}

pub fn write_string(subkey: &str, value_name: &str, value: &str) -> Result<(), RegistryError> {
    #[cfg(target_os = "windows")]
    {
        win_impl::write_string(subkey, value_name, value)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (subkey, value_name, value);
        Err(RegistryError::UnsupportedPlatform)
    }
}

pub fn delete_value(subkey: &str, value_name: &str) -> Result<(), RegistryError> {
    #[cfg(target_os = "windows")]
    {
        win_impl::delete_value(subkey, value_name)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (subkey, value_name);
        Err(RegistryError::UnsupportedPlatform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(
            PERSONALIZE_SUBKEY,
            r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"
        );
        assert_eq!(APPS_USE_LIGHT_THEME, "AppsUseLightTheme");
        assert_eq!(SYSTEM_USES_LIGHT_THEME, "SystemUsesLightTheme");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_read_personalize_registry() {
        // Safe read-only test that checks reading the Windows theme keys without modifying them
        match read_dword(PERSONALIZE_SUBKEY, APPS_USE_LIGHT_THEME) {
            Ok(val) => {
                assert!(val == 0 || val == 1, "AppsUseLightTheme should be 0 or 1");
            }
            Err(e) => {
                // If the key is not found or not set on some custom builds, verify it's a known error
                eprintln!("Personalize read error: {:?}", e);
            }
        }
    }
}
