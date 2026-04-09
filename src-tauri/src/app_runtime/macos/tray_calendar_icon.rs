//! macOS 菜单栏托盘：模板位图（仅黑色 + 透明度，无任意彩色）。
//!
//! **逻辑绘制**对齐 [LunarBar `DateIconView`](https://github.com/LunarBar-app/LunarBar/blob/main/LunarBarMac/Sources/Shared/AppIconFactory.swift)（21×15 点、圆角/描边/字号等）。
//! **像素画布**宽高由用户在前端配置（默认 **42×36**，即 21×18 的 @2×）；日期图按 LunarBar **21×15** 点等比落在「槽内」并居中。
//! 槽高取 **15/18** 画布高：与 [LunarBar `DateIconView`](https://github.com/LunarBar-app/LunarBar/blob/main/LunarBarMac/Sources/Shared/AppIconFactory.swift) 的 **15pt** 图高、以及 tray 将整图压到约 **18pt** 高 对齐，使默认观感与 LunarBar 一致。
//!
//! **清晰度**：`tray-icon` 将 `NSImage` 的 **逻辑尺寸**固定为约 **18pt 高**（见 `set_icon_for_ns_status_item_button`）。用户配置的宽高默认 **42×36** 已表示 **21×18pt @2×**；若再乘 `backingScale`（Retina 上为 2）会得到 **@4×** 栅格，AppKit 再缩回 **@2×** 菜单栏时易糊。因此栅格只按「相对 2× 基准」补足：`factor = backingScale / 2`（1× 屏为 0.5×、2× 为 1×、3× 为 1.5×）。

use std::sync::OnceLock;

use fontdb::{Database, Family, Query, Weight};
use fontdue::{Font, Metrics};
use tauri::image::Image;

use crate::app_runtime::tray_icon_px::TrayIconPx;
use tiny_skia::{
    Color, FillRule, LineCap, LineJoin, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform,
};

/// 与 LunarBar `DateIconStyle` 对应：实心（整面填充 + 数字镂空）、描边（圆角框 + 数字）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TrayDateIconStyle {
    /// `filledDate`：粗体数字区域透明，其余为实心模板色。
    #[default]
    Filled,
    /// `outlinedDate`：圆角矩形描边，数字叠画于内。
    Outlined,
}

impl TrayDateIconStyle {
    pub fn from_config(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "outlined" | "outline" => Self::Outlined,
            _ => Self::Filled,
        }
    }

    pub fn as_config_str(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Outlined => "outlined",
        }
    }
}

/// LunarBar `DateIconView.Constants`（逻辑坐标，与图标比例一致）。
mod lunar_bar {
    pub const ICON_W: f32 = 21.0;
    pub const ICON_H: f32 = 15.0;
    pub const CORNER_RADIUS: f32 = 2.5;
    pub const BORDER_WIDTH: f32 = 2.0;
    pub const OUTLINED_BORDER_WIDTH: f32 = 1.25;
    pub const FONT_SIZE: f32 = 12.0;
    /// tray-icon 将状态栏图统一为约 18pt 高；LunarBar 日期图为 15pt 高 → 图形占画布高度 **15/18**。
    pub const TRAY_SLOT_H: f32 = 18.0;
}

static FONT_BOLD: OnceLock<Font> = OnceLock::new();
static FONT_REGULAR: OnceLock<Font> = OnceLock::new();

#[inline]
fn ink() -> Color {
    Color::from_rgba8(0, 0, 0, 255)
}

fn font_bold() -> &'static Font {
    FONT_BOLD.get_or_init(|| load_system_ui_font(Weight::BOLD))
}

fn font_regular() -> &'static Font {
    FONT_REGULAR.get_or_init(|| load_system_ui_font(Weight::NORMAL))
}

fn load_system_ui_font(weight: Weight) -> Font {
    let mut db = Database::new();
    db.load_system_fonts();

    let queries = [
        Query { families: &[Family::Name(".AppleSystemUIFont")], weight, ..Default::default() },
        Query { families: &[Family::Name("SF Pro Text")], weight, ..Default::default() },
        Query { families: &[Family::Name("Helvetica Neue")], weight, ..Default::default() },
        Query { families: &[Family::SansSerif], weight, ..Default::default() },
    ];

    for query in queries {
        if let Some(id) = db.query(&query) {
            if let Some(face) = db.face(id) {
                if let Some(bytes) = read_face_bytes(face) {
                    if let Ok(font) = Font::from_bytes(bytes, fontdue::FontSettings::default()) {
                        return font;
                    }
                }
            }
        }
    }

    for face in db.faces() {
        if let Some(bytes) = read_face_bytes(face) {
            if let Ok(font) = Font::from_bytes(bytes, fontdue::FontSettings::default()) {
                return font;
            }
        }
    }

    if let Ok(data) = std::fs::read("/System/Library/Fonts/Supplemental/Arial.ttf") {
        if let Ok(font) = Font::from_bytes(data, fontdue::FontSettings::default()) {
            return font;
        }
    }

    panic!("tray_calendar_icon: no TTF/OTF system font for menu bar");
}

fn read_face_bytes(face: &fontdb::FaceInfo) -> Option<Vec<u8>> {
    match &face.source {
        fontdb::Source::File(path) => {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !ext.eq_ignore_ascii_case("ttf") && !ext.eq_ignore_ascii_case("otf") {
                return None;
            }
            std::fs::read(path).ok()
        }
        fontdb::Source::Binary(_) => None,
        _ => None,
    }
}

pub fn calendar_tray_image(day: u32, style: TrayDateIconStyle, px: TrayIconPx) -> Image<'static> {
    let d = day.clamp(1, 31);
    let backing = main_screen_backing_scale().clamp(1.0, 4.0);
    let raster = physical_tray_raster_px_with_backing(px, backing);
    let rgba = render_rgba(d, style, raster);
    Image::new_owned(rgba, raster.width, raster.height)
}

/// 按主屏倍率补足像素密度；`px` 按 **@2×** 为默认基准（42×36），避免与 Retina 再乘一次 2 导致 @4× 栅格发糊。
pub(crate) fn physical_tray_raster_px_with_backing(px: TrayIconPx, backing: f32) -> TrayIconPx {
    let s = backing.clamp(1.0, 4.0);
    let factor = (s / 2.0).max(0.5);
    let w = ((px.width as f32) * factor).round() as u32;
    let h = ((px.height as f32) * factor).round() as u32;
    // 与 `tray_icon_px` 用户上限解耦：离屏可略大，避免极端倍率下过大分配。
    const MIN_RASTER: u32 = 16;
    const MAX_RASTER: u32 = 512;
    TrayIconPx { width: w.clamp(MIN_RASTER, MAX_RASTER), height: h.clamp(MIN_RASTER, MAX_RASTER) }
}

// 工程已依赖 `cocoa`；与窗口毛玻璃等一致，待统一迁 objc2 时再换 API。
#[allow(deprecated)]
pub(crate) fn main_screen_backing_scale() -> f32 {
    use cocoa::appkit::NSScreen;
    use cocoa::base::nil;

    unsafe {
        let screen = NSScreen::mainScreen(nil);
        if screen == nil {
            return 2.0;
        }
        screen.backingScaleFactor() as f32
    }
}

fn render_rgba(day: u32, style: TrayDateIconStyle, px: TrayIconPx) -> Vec<u8> {
    let mut pixmap = Pixmap::new(px.width, px.height).expect("tray pixmap");
    pixmap.fill(Color::TRANSPARENT);

    let pixmap_w = px.width as f32;
    let pixmap_h = px.height as f32;
    // 与 LunarBar 一致：日期图本身为 21×15「点」区域；整图在 tray 中约 18pt 高，故先取高度为画布 15/18 的内接带再套入 21:15。
    let inner_h = pixmap_h * (lunar_bar::ICON_H / lunar_bar::TRAY_SLOT_H);
    let inner_y = ((pixmap_h - inner_h) * 0.5).round();
    let inner_w_max = (lunar_bar::ICON_W / lunar_bar::ICON_H) * inner_h;
    let inner_w = pixmap_w.min(inner_w_max);
    let inner_x = ((pixmap_w - inner_w) * 0.5).round();

    let scale_fit = (inner_w / lunar_bar::ICON_W).min(inner_h / lunar_bar::ICON_H);
    // 内容区宽高与原点对齐到整像素，减轻子像素几何与 fontdue 非整数 px 栅格化带来的发糊。
    let content_w = (lunar_bar::ICON_W * scale_fit).round().clamp(1.0, inner_w);
    let content_h = (lunar_bar::ICON_H * scale_fit).round().clamp(1.0, inner_h);
    let ox = (inner_x + (inner_w - content_w) * 0.5).round();
    let oy = (inner_y + (inner_h - content_h) * 0.5).round();
    let bounds = Rect::from_xywh(ox, oy, content_w, content_h).expect("lunar bounds");

    // 用实际像素框反推缩放，使圆角/描边/字号与位图网格一致。
    let scale = (content_w / lunar_bar::ICON_W).min(content_h / lunar_bar::ICON_H);
    let corner = (lunar_bar::CORNER_RADIUS * scale * 4.0).round() / 4.0;
    let border_w = match style {
        TrayDateIconStyle::Filled => (lunar_bar::BORDER_WIDTH * scale).max(1.0).round(),
        TrayDateIconStyle::Outlined => {
            ((lunar_bar::OUTLINED_BORDER_WIDTH * scale).max(0.75) * 4.0).round() / 4.0
        }
    };
    let font_px = (lunar_bar::FONT_SIZE * scale).round().max(4.0);

    let text = day.to_string();

    match style {
        TrayDateIconStyle::Filled => render_filled(&mut pixmap, bounds, corner, font_px, &text),
        TrayDateIconStyle::Outlined => {
            render_outlined(&mut pixmap, bounds, corner, border_w, font_px, &text)
        }
    }

    premultiplied_to_straight_rgba(pixmap.data())
}

/// 位图左上角 Y（Y 轴向下、与屏幕一致）。fontdue 的 `Metrics::ymin` 是**底边**相对基线，不能当 top 用；
/// 与 `fontdue::layout` 在 `CoordinateSystem::PositiveYDown` 下一致：`floor(-bounds.height - bounds.ymin)`。
fn glyph_bitmap_top_y(baseline_y: f32, m: &Metrics) -> f32 {
    baseline_y + (-m.bounds.height - m.bounds.ymin).floor()
}

/// 在 `bounds` 内将一行文字垂直居中时的基线 Y（`descent` 通常为负）。
fn baseline_y_centered_in_rect(bounds: Rect, line: &fontdue::LineMetrics) -> f32 {
    let center_y = bounds.y() + bounds.height() * 0.5;
    center_y + (line.ascent + line.descent) * 0.5
}

/// 对齐到半像素，减轻字形与路径相对位图网格的次像素偏移。
#[inline]
fn snap_half_px(v: f32) -> f32 {
    (v * 2.0).round() / 2.0
}

/// LunarBar `renderFilledIcon`：圆角矩形实心 + 粗体数字镂空（mask 等价于从实心中扣除字形）。
fn render_filled(pixmap: &mut Pixmap, bounds: Rect, corner: f32, font_px: f32, text: &str) {
    let mut pb = PathBuilder::new();
    push_rounded_rect(&mut pb, bounds, corner);
    let Some(path) = pb.finish() else {
        return;
    };
    let mut paint = Paint::default();
    paint.set_color(ink());
    paint.anti_alias = true;
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let f = font_bold();
    let line = f.horizontal_line_metrics(font_px).unwrap_or(fontdue::LineMetrics {
        ascent: font_px * 0.8,
        descent: -font_px * 0.2,
        line_gap: 0.0,
        new_line_size: font_px,
    });

    let mut total_w = 0.0_f32;
    for ch in text.chars() {
        let (m, _) = f.rasterize(ch, font_px);
        total_w += m.advance_width;
    }

    let baseline_y = snap_half_px(baseline_y_centered_in_rect(bounds, &line));

    let start_x = snap_half_px(bounds.x() + (bounds.width() - total_w) * 0.5);

    let pw = pixmap.width();
    let ph = pixmap.height();
    let buf = pixmap.data_mut();
    let mut x = start_x;
    for ch in text.chars() {
        let (metrics, bitmap) = f.rasterize(ch, font_px);
        let ox = x + metrics.bounds.xmin.floor();
        let oy = glyph_bitmap_top_y(baseline_y, &metrics);
        punch_out_glyph_coverage(buf, pw, ph, &bitmap, metrics.width, metrics.height, ox, oy);
        x += metrics.advance_width;
    }
}

/// LunarBar `renderOutlinedIcon`：`NSBezierPath(roundedRect: bounds, xRadius:yRadius:)` + `lineWidth` + `stroke`，数字 `systemFont` 居中。
///
/// AppKit 以路径为**中心线**描边；若路径贴齐位图边缘，外侧一半会被裁掉，左右与上下边粗细观感不一致。
/// 将圆角矩形路径向内缩进 `border_w/2` 再描边，使整条线落在 `bounds` 内，四边与四角粗细与 LunarBar 一致。
fn render_outlined(
    pixmap: &mut Pixmap,
    bounds: Rect,
    corner: f32,
    border_w: f32,
    font_px: f32,
    text: &str,
) {
    let (stroke_bounds, stroke_corner) =
        inset_bounds_for_centered_stroke(bounds, corner, border_w).unwrap_or((bounds, corner));

    let mut pb = PathBuilder::new();
    push_rounded_rect(&mut pb, stroke_bounds, stroke_corner);
    let Some(path) = pb.finish() else {
        return;
    };
    let mut paint = Paint::default();
    paint.set_color(ink());
    paint.anti_alias = true;
    let stroke = Stroke {
        width: border_w,
        line_cap: LineCap::Butt,
        line_join: LineJoin::Round,
        ..Stroke::default()
    };
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

    let f = font_regular();
    let line = f.horizontal_line_metrics(font_px).unwrap_or(fontdue::LineMetrics {
        ascent: font_px * 0.8,
        descent: -font_px * 0.2,
        line_gap: 0.0,
        new_line_size: font_px,
    });

    let mut total_w = 0.0_f32;
    for ch in text.chars() {
        let (m, _) = f.rasterize(ch, font_px);
        total_w += m.advance_width;
    }

    let baseline_y = snap_half_px(baseline_y_centered_in_rect(bounds, &line));
    let start_x = snap_half_px(bounds.x() + (bounds.width() - total_w) * 0.5);

    let pw = pixmap.width();
    let ph = pixmap.height();
    let buf = pixmap.data_mut();
    let mut x = start_x;
    for ch in text.chars() {
        let (metrics, bitmap) = f.rasterize(ch, font_px);
        let ox = x + metrics.bounds.xmin.floor();
        let oy = glyph_bitmap_top_y(baseline_y, &metrics);
        blend_glyph_coverage(buf, pw, ph, &bitmap, metrics.width, metrics.height, ox, oy);
        x += metrics.advance_width;
    }
}

/// 与 `NSBezierPath` 居中描边配合：路径为向内缩进 `border_w/2` 的圆角矩形，圆角半径同步减小（平行偏移近似）。
fn inset_bounds_for_centered_stroke(
    bounds: Rect,
    corner: f32,
    border_w: f32,
) -> Option<(Rect, f32)> {
    let d = border_w * 0.5;
    let w = bounds.width() - border_w;
    let h = bounds.height() - border_w;
    if !(w > 0.0 && h > 0.0) {
        return None;
    }
    let x = bounds.x() + d;
    let y = bounds.y() + d;
    let stroke_corner = (corner - d).max(0.0).min(w * 0.5).min(h * 0.5);
    let stroke_corner = (stroke_corner * 4.0).round() / 4.0;
    let rect = Rect::from_xywh(x, y, w, h)?;
    Some((rect, stroke_corner))
}

/// 顺时针圆角矩形路径（近似 `NSBezierPath(roundedRect:xRadius:yRadius:)`）。
fn push_rounded_rect(pb: &mut PathBuilder, r: Rect, radius: f32) {
    let radius = radius.min(r.width() * 0.5).min(r.height() * 0.5);
    let x = r.left();
    let y = r.top();
    let w = r.width();
    let h = r.height();
    let k = 0.552_284_8 * radius;

    pb.move_to(x + radius, y);
    pb.line_to(x + w - radius, y);
    pb.cubic_to(x + w - radius + k, y, x + w, y + radius - k, x + w, y + radius);
    pb.line_to(x + w, y + h - radius);
    pb.cubic_to(x + w, y + h - radius + k, x + w - radius + k, y + h, x + w - radius, y + h);
    pb.line_to(x + radius, y + h);
    pb.cubic_to(x + radius - k, y + h, x, y + h - radius + k, x, y + h - radius);
    pb.line_to(x, y + radius);
    pb.cubic_to(x, y + radius - k, x + radius - k, y, x + radius, y);
    pb.close();
}

#[allow(clippy::too_many_arguments)]
fn blend_glyph_coverage(
    data: &mut [u8],
    w: u32,
    h: u32,
    bitmap: &[u8],
    bw: usize,
    bh: usize,
    origin_x: f32,
    origin_y: f32,
) {
    for row in 0..bh {
        for col in 0..bw {
            let cov = bitmap[row * bw + col];
            if cov == 0 {
                continue;
            }
            let px = origin_x + col as f32;
            let py = origin_y + row as f32;
            let xi = px.round() as i32;
            let yi = py.round() as i32;
            if xi < 0 || yi < 0 {
                continue;
            }
            let xi = xi as u32;
            let yi = yi as u32;
            if xi >= w || yi >= h {
                continue;
            }
            let idx = ((yi * w + xi) * 4) as usize;
            blend_black_premul_over(data, idx, cov as u32);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn punch_out_glyph_coverage(
    data: &mut [u8],
    w: u32,
    h: u32,
    bitmap: &[u8],
    bw: usize,
    bh: usize,
    origin_x: f32,
    origin_y: f32,
) {
    for row in 0..bh {
        for col in 0..bw {
            let cov = bitmap[row * bw + col];
            if cov == 0 {
                continue;
            }
            let px = origin_x + col as f32;
            let py = origin_y + row as f32;
            let xi = px.round() as i32;
            let yi = py.round() as i32;
            if xi < 0 || yi < 0 {
                continue;
            }
            let xi = xi as u32;
            let yi = yi as u32;
            if xi >= w || yi >= h {
                continue;
            }
            let idx = ((yi * w + xi) * 4) as usize;
            punch_out_black_premul(data, idx, cov as u32);
        }
    }
}

fn blend_black_premul_over(dst: &mut [u8], idx: usize, sa: u32) {
    let dr = dst[idx] as u32;
    let dg = dst[idx + 1] as u32;
    let db = dst[idx + 2] as u32;
    let da = dst[idx + 3] as u32;
    let inv = 255 - sa;
    dst[idx] = ((dr * inv) / 255) as u8;
    dst[idx + 1] = ((dg * inv) / 255) as u8;
    dst[idx + 2] = ((db * inv) / 255) as u8;
    dst[idx + 3] = ((sa * 255 + da * inv) / 255).min(255) as u8;
}

fn punch_out_black_premul(dst: &mut [u8], idx: usize, glyph_alpha: u32) {
    let t = 255u32.saturating_sub(glyph_alpha);
    let a = dst[idx + 3] as u32;
    let r = dst[idx] as u32;
    let g = dst[idx + 1] as u32;
    let b = dst[idx + 2] as u32;
    let na = (a * t) / 255;
    dst[idx] = ((r * t) / 255) as u8;
    dst[idx + 1] = ((g * t) / 255) as u8;
    dst[idx + 2] = ((b * t) / 255) as u8;
    dst[idx + 3] = na.min(255) as u8;
}

pub(crate) fn premultiplied_to_straight_rgba(premul: &[u8]) -> Vec<u8> {
    premul
        .chunks_exact(4)
        .flat_map(|c| {
            let a = c[3] as u32;
            if a == 0 {
                [0u8, 0, 0, 0]
            } else {
                let r = ((c[0] as u32 * 255 + a / 2) / a).min(255) as u8;
                let g = ((c[1] as u32 * 255 + a / 2) / a).min(255) as u8;
                let b = ((c[2] as u32 * 255 + a / 2) / a).min(255) as u8;
                [r, g, b, a as u8]
            }
        })
        .collect()
}
