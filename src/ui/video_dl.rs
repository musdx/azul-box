use eframe::egui::{self, Button, Color32};
use native_dialog::DialogBuilder;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicI8, Ordering};

use crate::ui::shares::notify::{
    button_sound, done_sound, fail_sound, notification_done, notification_fail,
};

pub struct VideoDownload {
    pub link: String,
    pub out_directory: String,
    pub status: Arc<AtomicI8>,
    pub format: i8,
    pub frag: i8,
    pub subtitle: bool,
    pub sub_lang: String,
}

impl Default for VideoDownload {
    fn default() -> Self {
        let default_directory = dirs::video_dir()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| String::from(""));
        Self {
            link: String::new(),
            out_directory: default_directory,
            status: Arc::new(AtomicI8::new(0)), // 0 = nothing / 1 = pending / 2 = Done / 3 = Fail
            format: 1,
            frag: 1,
            subtitle: true,
            sub_lang: "en".to_string(),
        }
    }
}

impl VideoDownload {
    fn start_download_status(&mut self) {
        self.status.store(1, Ordering::Relaxed);
    }
    fn format_button(&mut self, ui: &mut egui::Ui, name: &str, numbername: i8) {
        if self.format == numbername {
            if ui
                .add(egui::Button::new(
                    egui::RichText::new(name).color(Color32::LIGHT_BLUE),
                ))
                .clicked()
            {
                self.format = numbername;
                ui.close_menu();
            };
        } else {
            if ui.button(name).clicked() {
                self.format = numbername;
                ui.close_menu();
            };
        }
    }
    fn sub_button(&mut self, ui: &mut egui::Ui, code: String, lang: String) {
        if self.sub_lang == code {
            if ui
                .add(egui::Button::new(
                    egui::RichText::new(&lang).color(Color32::LIGHT_BLUE),
                ))
                .clicked()
            {
                self.sub_lang = code;
                ui.close_menu();
            };
        } else {
            if ui.button(&lang).clicked() {
                self.sub_lang = code;
                ui.close_menu();
            }
        }
    }
    fn lang_choice(&mut self, ui: &mut egui::Ui) {
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
                self.sub_button(ui, code.to_string(), lang.to_string());
            }
        });
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("Setting", |ui| {
                ui.menu_button("Format", |ui| {
                    self.format_button(ui, "MKV", 1);
                    self.format_button(ui, "MP4", 2);
                });

                ui.add(egui::widgets::Slider::new(&mut self.frag, 1..=10).text("Fragments"));
                if self.subtitle {
                    if ui
                        .add(Button::new(
                            egui::RichText::new("Subtitle").color(Color32::LIGHT_BLUE),
                        ))
                        .clicked()
                    {
                        self.subtitle = false;
                    }
                    self.lang_choice(ui);
                } else {
                    if ui.button("Subtitle").clicked() {
                        self.subtitle = true;
                    }
                }
                if ui.button("Close").clicked() {
                    ui.close_menu();
                }
            });
            ui.label("Status: ");
            if self.status.load(Ordering::Relaxed) == 1 {
                ui.spinner();
            } else if self.status.load(Ordering::Relaxed) == 2 {
                ui.colored_label(Color32::LIGHT_GREEN, "Done!");
            } else if self.status.load(Ordering::Relaxed) == 3 {
                ui.colored_label(Color32::LIGHT_RED, "Fail!");
            }
        });
        ui.separator();
        ui.vertical_centered(|ui| {
            let link_label = ui.label("Link: ");
            ui.text_edit_singleline(&mut self.link)
                .labelled_by(link_label.id);

            let dir_label = ui.label("Directory: ");
            if ui
                .text_edit_singleline(&mut self.out_directory)
                .labelled_by(dir_label.id)
                .clicked()
            {
                let path = DialogBuilder::file()
                    .set_location(&self.out_directory)
                    .open_single_dir()
                    .show()
                    .unwrap();

                if let Some(p) = path {
                    self.out_directory = p.to_string_lossy().into_owned();
                } else {
                    println!("No file selected.");
                }
            };

            if ui.button("Download").clicked() {
                button_sound();
                if !(self.status.load(Ordering::Relaxed) == 1) {
                    self.start_download_status();

                    let link = self.link.clone();
                    let directory = self.out_directory.clone();
                    let format = self.format.clone();
                    let frags = self.frag.clone();
                    let progress = self.status.clone();
                    let subtile = self.subtitle.clone();
                    let lang = self.sub_lang.clone();

                    tokio::task::spawn(async move {
                        let status = download(link, directory, format, frags, subtile, &lang);
                        progress.store(status, Ordering::Relaxed);
                        if status == 2 {
                            done_sound();
                        } else {
                            fail_sound();
                        }
                    });
                }
            }
        });
    }
}

fn download(link: String, directory: String, format: i8, frag: i8, sub: bool, lang: &str) -> i8 {
    let n = frag.to_string().to_owned();

    let mut yt = Command::new("yt-dlp");
    yt.arg("--concurrent-fragments")
        .arg(n)
        .arg("--embed-thumbnail")
        .arg("--embed-metadata")
        .current_dir(directory);
    if sub {
        yt.arg("--embed-subs").arg("--sub-lang").arg(lang);
    }

    if format == 1 {
        yt.arg("-f").arg("bestvideo+bestaudio");
    } else if format == 2 {
        yt.arg("-f")
            .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best");
    }
    let output = yt.arg(link).output().expect("Pls some thing");
    let log = String::from_utf8(output.stdout).unwrap_or_else(|_| "Life suck".to_string());
    println!("{log}");

    let status: i8 = if log.contains("[EmbedThumbnail]") {
        2
    } else {
        3
    };

    if status == 2 {
        let _ = notification_done("video downloader");
    } else {
        let _ = notification_fail("video downloader");
    }
    status
}
