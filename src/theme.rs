use three_d::egui::{self, Color32, Rounding, Stroke, Style, Visuals};

/// Fusion 360-inspired color palette
#[allow(dead_code)]
pub struct FusionColors;

#[allow(dead_code)]
impl FusionColors {
    // Background colors
    pub const CANVAS_BG: [f32; 3] = [0.18, 0.20, 0.22]; // Dark gray canvas
    pub const PANEL_BG: Color32 = Color32::from_rgb(45, 48, 52); // Slightly lighter panel
    pub const PANEL_BG_DARK: Color32 = Color32::from_rgb(35, 38, 42);
    pub const HEADER_BG: Color32 = Color32::from_rgb(38, 41, 45);

    // Accent colors (Fusion 360 uses orange/amber accents)
    pub const ACCENT: Color32 = Color32::from_rgb(255, 153, 0); // Orange accent
    pub const ACCENT_HOVER: Color32 = Color32::from_rgb(255, 180, 60);
    pub const ACCENT_DIM: Color32 = Color32::from_rgb(180, 108, 0);

    // Text colors
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(230, 230, 230);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(160, 165, 170);
    pub const TEXT_DIM: Color32 = Color32::from_rgb(110, 115, 120);

    // Widget colors
    pub const WIDGET_BG: Color32 = Color32::from_rgb(55, 60, 65);
    pub const WIDGET_BG_HOVER: Color32 = Color32::from_rgb(70, 75, 82);
    pub const WIDGET_BG_ACTIVE: Color32 = Color32::from_rgb(80, 85, 92);
    pub const WIDGET_BORDER: Color32 = Color32::from_rgb(75, 80, 88);

    // Slider specific
    pub const SLIDER_RAIL: Color32 = Color32::from_rgb(50, 55, 60);
    pub const SLIDER_FILL: Color32 = Color32::from_rgb(255, 153, 0);

    // Grid colors
    pub const GRID_MAJOR: Color32 = Color32::from_rgba_premultiplied(80, 85, 95, 120);
    pub const GRID_MINOR: Color32 = Color32::from_rgba_premultiplied(60, 65, 72, 80);

    // Axis colors (matching Fusion 360)
    pub const AXIS_X: Color32 = Color32::from_rgb(230, 75, 60);  // Red
    pub const AXIS_Y: Color32 = Color32::from_rgb(75, 180, 75);  // Green
    pub const AXIS_Z: Color32 = Color32::from_rgb(65, 130, 230); // Blue

    // Selection and highlight
    pub const SELECTION: Color32 = Color32::from_rgb(60, 140, 220);
    pub const HIGHLIGHT: Color32 = Color32::from_rgb(100, 180, 255);
}

/// Apply Fusion 360-inspired theme to egui
pub fn apply_fusion_theme(ctx: &egui::Context) {
    let mut style = Style::default();

    // Spacing adjustments for cleaner look
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(12.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    style.spacing.indent = 18.0;
    style.spacing.slider_width = 140.0;
    style.spacing.combo_width = 120.0;

    // Rounding for modern feel
    style.visuals.window_rounding = Rounding::same(6.0);
    style.visuals.menu_rounding = Rounding::same(4.0);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(4.0);
    style.visuals.widgets.inactive.rounding = Rounding::same(4.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(4.0);
    style.visuals.widgets.active.rounding = Rounding::same(4.0);
    style.visuals.widgets.open.rounding = Rounding::same(4.0);

    // Dark theme colors
    let mut visuals = Visuals::dark();

    // Panel backgrounds
    visuals.panel_fill = FusionColors::PANEL_BG;
    visuals.window_fill = FusionColors::PANEL_BG;
    visuals.extreme_bg_color = FusionColors::PANEL_BG_DARK;
    visuals.faint_bg_color = Color32::from_rgb(40, 43, 48);

    // Widget styling
    visuals.widgets.noninteractive.bg_fill = FusionColors::WIDGET_BG;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, FusionColors::TEXT_SECONDARY);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, FusionColors::WIDGET_BORDER);

    visuals.widgets.inactive.bg_fill = FusionColors::WIDGET_BG;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, FusionColors::TEXT_PRIMARY);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, FusionColors::WIDGET_BORDER);

    visuals.widgets.hovered.bg_fill = FusionColors::WIDGET_BG_HOVER;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, FusionColors::TEXT_PRIMARY);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, FusionColors::ACCENT);

    visuals.widgets.active.bg_fill = FusionColors::WIDGET_BG_ACTIVE;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, FusionColors::TEXT_PRIMARY);
    visuals.widgets.active.bg_stroke = Stroke::new(2.0, FusionColors::ACCENT);

    visuals.widgets.open.bg_fill = FusionColors::WIDGET_BG_ACTIVE;
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, FusionColors::TEXT_PRIMARY);
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, FusionColors::ACCENT);

    // Selection highlight
    visuals.selection.bg_fill = FusionColors::ACCENT.linear_multiply(0.4);
    visuals.selection.stroke = Stroke::new(1.0, FusionColors::ACCENT);

    // Hyperlink color
    visuals.hyperlink_color = FusionColors::ACCENT;

    // Window shadow for depth
    visuals.window_shadow = egui::epaint::Shadow {
        extrusion: 12.0,
        color: Color32::from_rgba_premultiplied(0, 0, 0, 80),
    };

    // Popup shadow
    visuals.popup_shadow = egui::epaint::Shadow {
        extrusion: 8.0,
        color: Color32::from_rgba_premultiplied(0, 0, 0, 60),
    };

    // Separator color
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(60, 65, 72));

    style.visuals = visuals;
    ctx.set_style(style);
}

/// Create a styled slider that looks like Fusion 360
#[allow(dead_code)]
pub fn fusion_slider<'a>(value: &'a mut f32, range: std::ops::RangeInclusive<f32>) -> egui::Slider<'a> {
    egui::Slider::new(value, range)
        .show_value(true)
        .trailing_fill(true)
}

/// Create a styled drag value
#[allow(dead_code)]
pub fn fusion_drag_value(value: &mut f32, speed: f32) -> egui::DragValue<'_> {
    egui::DragValue::new(value)
        .speed(speed)
        .min_decimals(2)
        .max_decimals(3)
}
