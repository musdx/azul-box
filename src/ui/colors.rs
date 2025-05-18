use eframe::egui::{self, Color32, TextEdit};

pub struct Colors {
    pub color_srgba: Color32,
    pub hex_input: String,
}

impl Default for Colors {
    fn default() -> Self {
        let default_color = Color32::from_rgb(0, 0, 0);
        Self {
            color_srgba: default_color,
            hex_input: format!(
                "#{:02X}{:02X}{:02X}",
                default_color.r(),
                default_color.g(),
                default_color.b()
            ),
        }
    }
}

impl Colors {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.colored_label(self.color_srgba, "SRGBA:");
            ui.add_space(10.0);
            let changed = ui.color_edit_button_srgba(&mut self.color_srgba).changed();
            if changed {
                self.hex_input = format!(
                    "#{:02X}{:02X}{:02X}",
                    self.color_srgba.r(),
                    self.color_srgba.g(),
                    self.color_srgba.b()
                );
            }
            let (r, g, b, a) = (
                self.color_srgba.r(),
                self.color_srgba.g(),
                self.color_srgba.b(),
                self.color_srgba.a(),
            );
            ui.label(format!("rgba({}, {}, {}, {})", r, g, b, a));
        });
        ui.horizontal(|ui| {
            ui.colored_label(self.color_srgba, "Hex:");
            ui.add_space(20.0);
            let response = ui.add_sized(
                [100.0, 20.0],
                TextEdit::singleline(&mut self.hex_input).background_color(self.color_srgba),
            );

            if response.changed() {
                if let Some(parsed_color) = parse_hex_color(&self.hex_input) {
                    self.color_srgba = parsed_color;
                }
            }
        });
    }
}

fn parse_hex_color(hex: &str) -> Option<Color32> {
    let hex = hex.trim_start_matches('#');
    u32::from_str_radix(hex, 16).ok().map(|rgb| {
        let r = ((rgb >> 16) & 0xFF) as u8;
        let g = ((rgb >> 8) & 0xFF) as u8;
        let b = (rgb & 0xFF) as u8;
        Color32::from_rgb(r, g, b)
    })
}
