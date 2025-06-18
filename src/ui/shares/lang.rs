use eframe::egui::{self, Color32, Ui};

pub struct lang_thing {}

impl lang_thing {
    pub fn lang_chooser(ui: &mut Ui, mut lang_in: String) -> String {
        let language_codes: Vec<&str> = vec![
            "en", // English
            "fr", // French
            "es", // Spanish
            "zh", // Chinese
            "de", // German
            "ja", // Japanese
            "ar", // Arabic
            "ru", // Russian
            "it", // Italian
            "pt", // Portuguese
            "nl", // Dutch
            "sv", // Swedish
            "no", // Norwegian
            "fi", // Finnish
            "da", // Danish
            "pl", // Polish
            "cs", // Czech
            "hu", // Hungarian
            "ro", // Romanian
            "tr", // Turkish
            "vi", // Vietnamese
            "ko", //Korean
        ];
        let languages: Vec<&str> = vec![
            "English",
            "French",
            "Spanish",
            "Chinese",
            "German",
            "Japanese",
            "Arabic",
            "Russian",
            "Italian",
            "Portuguese",
            "Dutch",
            "Swedish",
            "Norwegian",
            "Finnish",
            "Danish",
            "Polish",
            "Czech",
            "Hungarian",
            "Romanian",
            "Turkish",
            "Vietnamese",
            "Korean",
        ];
        ui.menu_button("Languages", |ui| {
            for (lang, code) in languages.iter().zip(language_codes.iter()) {
                if lang_in == code.to_string() {
                    if ui
                        .add(egui::Button::new(
                            egui::RichText::new(*lang).color(Color32::LIGHT_BLUE),
                        ))
                        .clicked()
                    {
                        lang_in = code.to_string();
                        ui.close_menu();
                    };
                } else {
                    if ui.button(*lang).clicked() {
                        lang_in = code.to_string();
                        ui.close_menu();
                    }
                }
            }
        });
        lang_in
    }
}
