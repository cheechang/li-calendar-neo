//! Tauri 原生可执行入口：在非调试构建下隐藏 Windows 控制台子系统。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// 程序入口，转调库 crate 中的 `run`。
fn main() {
    li_calendar_lib::run();
}
