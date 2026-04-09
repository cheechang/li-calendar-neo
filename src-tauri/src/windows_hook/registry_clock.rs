//! 注册表读写、自定义任务栏时钟文本与系统时间格式同步。

use windows::core::*;
use windows::Win32::Graphics::Gdi::{InvalidateRect, UpdateWindow};
use windows::Win32::System::Registry::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::{
    Foundation::*,
    Globalization::{
        GetLocaleInfoW, LOCALE_NOUSEROVERRIDE, LOCALE_SSHORTDATE, LOCALE_SSHORTTIME,
        LOCALE_USER_DEFAULT,
    },
};

use super::clock_window::find_clock_window;
use super::state::{CLOCK_TEXT_CACHE, CUSTOM_CLOCK_ENABLED, CUSTOM_CLOCK_TEXT};

/// 辅助函数：向注册表写入 DWORD 值。
///
/// * `key` - 注册表键句柄
/// * `name` - 值名称
/// * `value` - 要写入的 DWORD 值
pub(super) unsafe fn set_registry_dword(key: HKEY, name: PCWSTR, value: u32) {
    let _ = RegSetValueExW(
        key,
        name,
        Some(0),
        REG_DWORD,
        Some(std::slice::from_raw_parts(
            &value as *const u32 as *const u8,
            std::mem::size_of::<u32>(),
        )),
    );
}

/// 辅助函数：向注册表写入字符串值。
///
/// * `key` - 注册表键句柄
/// * `name` - 值名称
/// * `value` - 要写入的字符串值
pub(super) unsafe fn set_registry_string(key: HKEY, name: PCWSTR, value: &HSTRING) {
    let _ = RegSetValueExW(
        key,
        name,
        Some(0),
        REG_SZ,
        Some(std::slice::from_raw_parts(value.as_ptr().cast::<u8>(), value.len() * 2 + 2)),
    );
}

/// 向当前用户下指定 `subkey` 写入短时间/长时间/短日期/长日期格式字符串。
///
/// * `subkey` - 如 `Control Panel\\International`。
/// * `time_part` - 写入 `sShortTime` / `sTimeFormat` / `sLongTime`。
/// * `date_part` - 为 `Some` 时同时写 `sShortDate` / `sLongDate`。
fn write_time_date_to_registry(subkey: PCWSTR, time_part: &str, date_part: Option<&str>) {
    unsafe {
        let hkey = HKEY_CURRENT_USER;
        let mut key = HKEY::default();

        if RegOpenKeyExW(hkey, subkey, Some(0), KEY_ALL_ACCESS, &mut key).is_ok() {
            // 与系统控制面板一致，多写 `sTimeFormat`/`sLongTime` 避免 Win11 混用
            let time_w = HSTRING::from(time_part);

            set_registry_string(key, w!("sShortTime"), &time_w);
            set_registry_string(key, w!("sTimeFormat"), &time_w);
            set_registry_string(key, w!("sLongTime"), &time_w);

            if let Some(date) = date_part {
                let date_w = HSTRING::from(date);
                set_registry_string(key, w!("sShortDate"), &date_w);
                set_registry_string(key, w!("sLongDate"), &date_w);
            }

            let _ = RegFlushKey(key);
            let _ = RegCloseKey(key);
        }
    }
}

/// 读取 `name` 指向的 `REG_SZ`；失败或长度异常时返回 `default`。
///
/// * `key` - 已打开的注册表键。
/// * `name` - 值名宽字符串。
/// * `default` - 回退默认（多为格式占位如 `HH:mm`）。
fn read_reg_string(key: HKEY, name: windows::core::PCWSTR, default: &str) -> String {
    unsafe {
        let mut data = [0u16; 512];
        let mut len = (data.len() * 2) as u32;
        if RegQueryValueExW(
            key,
            name,
            None,
            None,
            Some(data.as_mut_ptr() as *mut u8),
            Some(&mut len),
        )
        .is_ok()
        {
            let u16_len = (len / 2) as usize;
            let slice = if u16_len > 0 && data[u16_len - 1] == 0 {
                &data[..u16_len - 1]
            } else {
                &data[..u16_len]
            };
            String::from_utf16_lossy(slice)
        } else {
            default.to_string()
        }
    }
}

/// 辅助函数：安全读取注册表 DWORD 值。
///
/// * `key` - 注册表键句柄
/// * `name` - 值名称
/// * `default` - 默认值
pub(super) unsafe fn read_reg_dword(key: HKEY, name: windows::core::PCWSTR, default: u32) -> u32 {
    let mut current_value: u32 = 0;
    let mut len = std::mem::size_of::<u32>() as u32;
    if RegQueryValueExW(
        key,
        name,
        None,
        None,
        Some(&mut current_value as *mut u32 as *mut u8),
        Some(&mut len),
    )
    .is_ok()
    {
        current_value
    } else {
        default
    }
}

/// 历史遗留：曾用于共享内存映射自定义时钟文本；现已无操作，保留以兼容调用点。
fn clear_clock_text_mapping() {
    // 共享内存映射逻辑已移除，此处保留空函数以兼容现有调用
}

/// 使任务栏时钟控件立即重绘，并在必要时翻转「系统时钟显示秒」以强制刷新。
fn force_refresh_clock_window() {
    unsafe {
        println!("🔄 执行任务栏时钟刷新...");

        if let Some(clock_hwnd) = find_clock_window() {
            println!("📍 找到时钟窗口，执行直接重绘");
            let _ = InvalidateRect(Some(clock_hwnd), None, true);
            let _ = UpdateWindow(clock_hwnd);

            if let Ok(parent) = GetParent(clock_hwnd) {
                if !parent.0.is_null() {
                    let _ = InvalidateRect(Some(parent), None, true);
                    let _ = UpdateWindow(parent);
                }
            }
        }

        let hkey = HKEY_CURRENT_USER;
        // 资源管理器「高级」项下的「在系统时钟中显示秒」开关
        let subkey = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced");
        let mut key = HKEY::default();

        if RegOpenKeyExW(hkey, subkey, Some(0), KEY_ALL_ACCESS, &mut key).is_ok() {
            let original_value = read_reg_dword(key, w!("ShowSecondsInSystemClock"), 0);

            // 临时翻转再恢复，促使 Shell 重绘时钟区域
            let temp_value: u32 = if original_value == 0 { 1 } else { 0 };

            println!("🔄 切换秒数显示进行强制刷新: {} -> {}", original_value, temp_value);

            set_registry_dword(key, w!("ShowSecondsInSystemClock"), temp_value);
            let _ = RegFlushKey(key);

            std::thread::sleep(std::time::Duration::from_millis(100));

            set_registry_dword(key, w!("ShowSecondsInSystemClock"), original_value);
            let _ = RegFlushKey(key);
            let _ = RegCloseKey(key);

            if let Some(clock_hwnd) = find_clock_window() {
                let _ = InvalidateRect(Some(clock_hwnd), None, true);
                let _ = UpdateWindow(clock_hwnd);
            }
        }

        println!("✅ 任务栏时钟刷新完成");
    }
}

/// 清除自定义时钟文本并恢复系统区域格式与注册表默认值（含广播 `WM_SETTINGCHANGE`）。
pub fn disable_custom_clock() -> Result<()> {
    if let Ok(mut enabled) = CUSTOM_CLOCK_ENABLED.lock() {
        *enabled = false;
    }
    if let Ok(mut text_guard) = CUSTOM_CLOCK_TEXT.lock() {
        *text_guard = None;
    }
    clear_clock_text_mapping();

    unsafe {
        // 使用 `LOCALE_NOUSEROVERRIDE` 取系统默认短时间与短日期格式（忽略当前用户覆盖层）
        let mut time_buf = [0u16; 80];
        let time_len = GetLocaleInfoW(
            LOCALE_USER_DEFAULT,
            LOCALE_SSHORTTIME | LOCALE_NOUSEROVERRIDE,
            Some(&mut time_buf),
        );
        let default_time = if time_len > 0 {
            String::from_utf16_lossy(&time_buf[..(time_len - 1) as usize])
        } else {
            "HH:mm".to_string()
        };

        let mut date_buf = [0u16; 80];
        let date_len = GetLocaleInfoW(
            LOCALE_USER_DEFAULT,
            LOCALE_SSHORTDATE | LOCALE_NOUSEROVERRIDE,
            Some(&mut date_buf),
        );
        let default_date = if date_len > 0 {
            String::from_utf16_lossy(&date_buf[..(date_len - 1) as usize])
        } else {
            "yyyy/M/d".to_string()
        };

        if let Ok(mut cache) = CLOCK_TEXT_CACHE.lock() {
            *cache = Some(format!("{}\n{}", default_time, default_date));
        }

        println!("获取到系统原始默认值 -> 时间: [{}], 日期: [{}]", default_time, default_date);

        write_time_date_to_registry(
            w!("Control Panel\\International"),
            &default_time,
            Some(&default_date),
        );
        write_time_date_to_registry(
            w!("Control Panel\\International\\User Profile"),
            &default_time,
            Some(&default_date),
        );

        let subkey_adv = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced");
        let mut key_adv = HKEY::default();
        if RegOpenKeyExW(HKEY_CURRENT_USER, subkey_adv, Some(0), KEY_ALL_ACCESS, &mut key_adv)
            .is_ok()
        {
            set_registry_dword(key_adv, w!("ShowSecondsInSystemClock"), 0);
            let _ = RegCloseKey(key_adv);
        }

        let _ = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(w!("intl").as_ptr() as isize),
            SMTO_ABORTIFHUNG,
            1000,
            None,
        );

        println!("已恢复为系统默认格式");

        force_refresh_clock_window();
    }
    Ok(())
}

/// 写入自定义时钟显示文本并同步到注册表与任务栏。
///
/// * `custom_text` - 多行文本；通常首段为时间格式，末行为日期格式（与设置页约定一致）。
pub fn set_custom_clock_text(custom_text: &str) -> Result<bool> {
    if let Ok(mut enabled) = CUSTOM_CLOCK_ENABLED.lock() {
        *enabled = true;
    }
    if let Ok(mut text_guard) = CUSTOM_CLOCK_TEXT.lock() {
        *text_guard = Some(custom_text.to_string());
    }
    if let Ok(mut cache) = CLOCK_TEXT_CACHE.lock() {
        *cache = Some(custom_text.replace("\r\n", "\n"));
    }

    let result = update_taskbar_clock_display()?;

    println!("🔄 立即强制刷新任务栏...");
    force_refresh_clock_window();

    Ok(result)
}

/// 返回当前应显示的时间/日期字符串（自定义文本优先，否则读 `Control Panel\\International` 并更新缓存）。
pub fn get_current_system_time_format() -> Result<String> {
    if let Ok(text_guard) = CUSTOM_CLOCK_TEXT.lock() {
        if let Some(custom_text) = text_guard.clone() {
            println!("返回用户自定义文本: {:?}", custom_text);
            return Ok(custom_text);
        }
    }

    if let Ok(cache) = CLOCK_TEXT_CACHE.lock() {
        if let Some(text) = cache.clone() {
            return Ok(text);
        }
    }

    unsafe {
        let hkey = HKEY_CURRENT_USER;
        let subkey = w!("Control Panel\\International");
        let mut key = HKEY::default();

        if RegOpenKeyExW(hkey, subkey, Some(0), KEY_READ, &mut key).is_ok() {
            let time_fmt = read_reg_string(key, w!("sShortTime"), "HH:mm");
            let date_fmt = read_reg_string(key, w!("sShortDate"), "yyyy/M/d");

            let _ = RegCloseKey(key);

            let text = format!("{}\n{}", time_fmt, date_fmt);
            if let Ok(mut cache) = CLOCK_TEXT_CACHE.lock() {
                *cache = Some(text.clone());
            }
            Ok(text)
        } else {
            println!("打开注册表失败，使用默认值");
            let text = "HH:mm\nyyyy/M/d".to_string();
            if let Ok(mut cache) = CLOCK_TEXT_CACHE.lock() {
                *cache = Some(text.clone());
            }
            Ok(text)
        }
    }
}

/// 在启用自定义时钟且文本非空时，将内容写入注册表（不单独调用 `force_refresh_clock_window`，由调用方决定）。
pub fn update_taskbar_clock_display() -> Result<bool> {
    let custom_text = {
        if let Ok(enabled) = CUSTOM_CLOCK_ENABLED.lock() {
            if !*enabled {
                println!("自定义时钟功能已禁用，跳过更新");
                return Ok(false);
            }
        }

        if let Ok(text_guard) = CUSTOM_CLOCK_TEXT.lock() {
            match text_guard.clone() {
                Some(text) => text,
                None => {
                    println!("未设置自定义时钟文本，跳过更新");
                    return Ok(false);
                }
            }
        } else {
            println!("无法获取自定义时钟文本，跳过更新");
            return Ok(false);
        }
    };

    println!("准备更新任务栏时钟显示: {:?}", custom_text);

    update_system_time_format(&custom_text)?;

    Ok(true)
}

/// 将多行自定义文本解析为时间与日期格式并写入注册表、同步「显示秒」开关。
///
/// * `custom_text` - 规范化换行后的完整用户输入。
pub fn update_system_time_format(custom_text: &str) -> Result<()> {
    unsafe {
        let sanitized = custom_text.replace("\r\n", "\n");
        if let Ok(mut cache) = CLOCK_TEXT_CACHE.lock() {
            *cache = Some(sanitized.clone());
        }
        let parts: Vec<&str> = sanitized.lines().collect();

        // 两行及以上：最后一行视为日期，前面合并为时间（允许多行时间格式）
        let (time_part, date_part) = if parts.len() >= 2 {
            let date = parts.last().copied().unwrap_or("");
            let time = parts[..parts.len() - 1].join("\n");
            (time, Some(date.to_string()))
        } else {
            (sanitized.clone(), None)
        };

        println!("即将写入注册表 -> sShortTime: [{}]", time_part);

        write_time_date_to_registry(
            w!("Control Panel\\International"),
            &time_part,
            date_part.as_deref(),
        );
        write_time_date_to_registry(
            w!("Control Panel\\International\\User Profile"),
            &time_part,
            date_part.as_deref(),
        );

        // 若格式串含秒，则打开「显示秒」注册表项，避免系统回写格式
        let has_seconds = time_part.contains(":ss") || time_part.contains(":s");
        let hkey = HKEY_CURRENT_USER;
        let subkey_adv = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced");
        let mut key_adv = HKEY::default();
        if RegOpenKeyExW(hkey, subkey_adv, Some(0), KEY_ALL_ACCESS, &mut key_adv).is_ok() {
            let show_seconds: u32 = if has_seconds { 1 } else { 0 };
            set_registry_dword(key_adv, w!("ShowSecondsInSystemClock"), show_seconds);
            let _ = RegCloseKey(key_adv);
            println!("已同步 ShowSecondsInSystemClock 为 {}", show_seconds);
        }

        let _ = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(w!("intl").as_ptr() as isize),
            SMTO_ABORTIFHUNG,
            1000,
            None,
        );

        println!("✅ 注册表写入并通知系统完成");
    }
    Ok(())
}
