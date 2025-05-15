use eframe::egui::{
    Style, Visuals,
    epaint::{Color32, Shadow, Stroke},
    style::{Selection, WidgetVisuals, Widgets},
};
pub const BASE_02: Color32 = Color32::from_rgb(188, 192, 204);
pub const BASE_03: Color32 = Color32::from_rgb(114, 135, 253);
pub const BASE_05: Color32 = Color32::from_rgb(76, 79, 105);
pub const BASE_06: Color32 = Color32::from_rgb(108, 111, 133);
pub const BASE_07: Color32 = Color32::from_rgb(92, 95, 119);
pub const BASE_08: Color32 = Color32::from_rgb(234, 118, 203);
pub const BASE_0A: Color32 = Color32::from_rgb(64, 160, 43);

const SHADOW: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 96);
const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);
const EXTREME_BACKGROUND: Color32 = Color32::from_rgb(204, 208, 218);
const MAIN_BACKGROUND: Color32 = Color32::from_rgb(239, 241, 245);
const MAIN_FOREGROUND: Color32 = BASE_02;
const CODE_BACKGROUND: Color32 = BASE_02;
const HOVERED_FILL: Color32 = BASE_02;
const HOVER_BACKGROUND: Color32 = BASE_03;
const SELECTION_STROKE: Color32 = Color32::from_rgb(114, 135, 253);
const UNUSABLE: Color32 = BASE_05;
const INACTIVE_STROKE: Color32 = BASE_05;
const OPEN_FILL: Color32 = BASE_06;
const HOVER: Color32 = BASE_06;
const ACTIVE_STROKE: Color32 = BASE_07;
const SELECTION_BACKGROUND: Color32 = BASE_08;
const HYPERLINK: Color32 = BASE_08;
const ACTIVE: Color32 = BASE_0A;
const ERROR_FOREGROUND: Color32 = Color32::from_rgb(249, 226, 175);
const WARNING_FOREGROUND: Color32 = Color32::from_rgb(223, 142, 29);

pub fn dark(original: Style) -> Style {
    Style {
        visuals: Visuals {
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: MAIN_BACKGROUND,
                    weak_bg_fill: MAIN_BACKGROUND,
                    bg_stroke: Stroke {
                        color: MAIN_FOREGROUND,
                        ..original.visuals.widgets.noninteractive.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: UNUSABLE,
                        ..original.visuals.widgets.noninteractive.fg_stroke
                    },
                    ..original.visuals.widgets.noninteractive
                },
                inactive: WidgetVisuals {
                    bg_fill: MAIN_FOREGROUND,
                    weak_bg_fill: MAIN_FOREGROUND,
                    bg_stroke: Stroke {
                        color: TRANSPARENT,
                        ..original.visuals.widgets.inactive.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: INACTIVE_STROKE,
                        ..original.visuals.widgets.inactive.fg_stroke
                    },
                    ..original.visuals.widgets.inactive
                },
                hovered: WidgetVisuals {
                    bg_fill: HOVERED_FILL,
                    weak_bg_fill: HOVERED_FILL,
                    bg_stroke: Stroke {
                        color: HOVER_BACKGROUND,
                        ..original.visuals.widgets.hovered.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: HOVER,
                        ..original.visuals.widgets.hovered.fg_stroke
                    },
                    ..original.visuals.widgets.hovered
                },
                active: WidgetVisuals {
                    bg_fill: ACTIVE,
                    weak_bg_fill: ACTIVE,
                    bg_stroke: Stroke {
                        color: ACTIVE_STROKE,
                        ..original.visuals.widgets.active.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: ACTIVE_STROKE,
                        ..original.visuals.widgets.active.fg_stroke
                    },
                    ..original.visuals.widgets.active
                },
                open: WidgetVisuals {
                    bg_fill: MAIN_BACKGROUND,
                    weak_bg_fill: MAIN_BACKGROUND,
                    bg_stroke: Stroke {
                        color: MAIN_FOREGROUND,
                        ..original.visuals.widgets.open.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: OPEN_FILL,
                        ..original.visuals.widgets.open.fg_stroke
                    },
                    ..original.visuals.widgets.open
                },
            },
            selection: Selection {
                bg_fill: SELECTION_BACKGROUND,
                stroke: Stroke {
                    color: SELECTION_STROKE,
                    ..original.visuals.selection.stroke
                },
            },
            hyperlink_color: HYPERLINK,
            faint_bg_color: TRANSPARENT,
            extreme_bg_color: EXTREME_BACKGROUND,
            code_bg_color: CODE_BACKGROUND,
            warn_fg_color: WARNING_FOREGROUND,
            error_fg_color: ERROR_FOREGROUND,
            window_shadow: Shadow {
                color: SHADOW,
                ..original.visuals.window_shadow
            },
            window_fill: MAIN_BACKGROUND,
            window_stroke: Stroke {
                color: MAIN_FOREGROUND,
                ..original.visuals.window_stroke
            },
            panel_fill: MAIN_BACKGROUND,
            popup_shadow: Shadow {
                color: SHADOW,
                ..original.visuals.popup_shadow
            },
            ..original.visuals
        },
        ..original
    }
}

pub fn theme() -> Style {
    let mut style = dark(Default::default()).clone();

    style
        .text_styles
        .get_mut(&eframe::egui::TextStyle::Heading)
        .unwrap()
        .size = 26.0;
    style
        .text_styles
        .get_mut(&eframe::egui::TextStyle::Body)
        .unwrap()
        .size = 20.0;
    style
        .text_styles
        .get_mut(&eframe::egui::TextStyle::Button)
        .unwrap()
        .size = 20.0;
    style
}
