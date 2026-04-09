//! 对齐 [LunarBar `AppIconFactory.createCalendarIcon`](https://github.com/LunarBar-app/LunarBar/blob/main/LunarBarMac/Sources/Shared/AppIconFactory.swift) 与 [`NSImage.with(symbolName:pointSize:weight:)`](https://github.com/LunarBar-app/LunarBar/blob/main/LunarBarMac/Modules/Sources/AppKitExtensions/NSImage+Extension.swift)：**16pt**、regular weight、模板图直接赋给 `NSStatusItem.button.image`。
#![allow(deprecated)]

use cocoa::base::{id, nil};
use cocoa::foundation::{NSSize, NSString};
use objc::rc::autoreleasepool;
use objc::runtime::Class;
use objc::{msg_send, sel, sel_impl};

/// 与 LunarBar `Icons.calendar` 一致。
const SYMBOL_NAME: &str = "calendar";
const SYMBOL_POINT_SIZE: f64 = 16.0;
const TRAY_ICON_LOGICAL_PT: f64 = 18.0;

pub fn apply_native_lunarbar_calendar_symbol_to_status_item(status_item: id) -> bool {
    if status_item == nil {
        return false;
    }
    autoreleasepool(|| unsafe { apply_native_inner(status_item) })
}

unsafe fn build_configured_symbol_image() -> Option<id> {
    let name = NSString::alloc(nil).init_str(SYMBOL_NAME);
    let cls = Class::get("NSImage")?;
    let img: id = msg_send![cls, imageWithSystemSymbolName: name accessibilityDescription: nil];
    if img == nil {
        return None;
    }

    let cfg_class = Class::get("NSImageSymbolConfiguration")?;
    let weight: f64 = 0.0;
    let config: id =
        msg_send![cfg_class, configurationWithPointSize: SYMBOL_POINT_SIZE weight: weight];
    let drawn: id = if config == nil {
        img
    } else {
        let i2: id = msg_send![img, imageWithSymbolConfiguration: config];
        if i2 == nil {
            img
        } else {
            i2
        }
    };
    Some(drawn)
}

unsafe fn apply_native_inner(status_item: id) -> bool {
    let button: id = msg_send![status_item, button];
    if button == nil {
        return false;
    }
    let drawn = match build_configured_symbol_image() {
        Some(img) => img,
        None => return false,
    };
    let _: () = msg_send![drawn, setTemplate: true];
    let side = SYMBOL_POINT_SIZE.min(TRAY_ICON_LOGICAL_PT);
    let size = NSSize::new(side, side);
    let _: () = msg_send![drawn, setSize: size];
    let image_left: i64 = 2;
    let _: () = msg_send![button, setImage: drawn];
    let _: () = msg_send![button, setImagePosition: image_left];
    true
}
