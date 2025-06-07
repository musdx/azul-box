use crate::ui::shares::notify::{button_sound, done_sound};
use eframe::egui::{self, Color32};
use native_dialog::DialogBuilder;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ImgConvert {
    pub out_directory: String,
    pub status_complete: Arc<AtomicBool>,
    pub status_pending: Arc<AtomicBool>,
    pub format_in: String,
    pub format_out: String,
    pub input_file: String,
}

impl Default for ImgConvert {
    fn default() -> Self {
        let default_directory = dirs::picture_dir()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| String::from(""));
        Self {
            input_file: String::new(),
            out_directory: default_directory,
            status_complete: Arc::new(AtomicBool::new(false)),
            status_pending: Arc::new(AtomicBool::new(false)),
            format_in: String::new(),
            format_out: String::from("None"),
        }
    }
}

impl ImgConvert {
    fn reset_download_status(&mut self) {
        self.status_complete.store(false, Ordering::Relaxed);
        self.status_pending.store(false, Ordering::Relaxed);
    }
    fn start_download_status(&mut self) {
        self.status_pending.store(true, Ordering::Relaxed);
    }
    fn format_out_button(&mut self, ui: &mut egui::Ui, name: &str) {
        if self.format_out == name && self.format_in != name {
            if ui
                .add(egui::Button::new(
                    egui::RichText::new(name).color(Color32::LIGHT_GREEN),
                ))
                .clicked()
            {
                self.format_out = name.to_string();
                ui.close_menu();
            };
        } else if self.format_in != name {
            if ui.button(name).clicked() {
                self.format_out = name.to_string();
                ui.close_menu();
            };
        }
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Output: ");
                ui.menu_button(self.format_out.clone(), |ui| {
                    self.format_out_button(ui, "jpg");
                    self.format_out_button(ui, "png");
                    self.format_out_button(ui, "bmp");
                    self.format_out_button(ui, "tif");
                    self.format_out_button(ui, "gif");
                    self.format_out_button(ui, "webp");
                    self.format_out_button(ui, "heic");
                    self.format_out_button(ui, "avif");
                });
            });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            ui.label("Status: ");
            if self.status_complete.load(Ordering::Relaxed) {
                ui.colored_label(Color32::GREEN, "Done!");
            } else if self.status_pending.load(Ordering::Relaxed) {
                ui.spinner();
            }
        });
        ui.separator();
        ui.vertical_centered(|ui| {
            let link_label = ui.label("Image: ");
            if ui
                .text_edit_singleline(&mut self.input_file)
                .labelled_by(link_label.id)
                .clicked()
            {
                let path = DialogBuilder::file()
                    .set_location(&self.out_directory)
                    .add_filter(
                        "Images",
                        &[
                            "jpg", "jpeg", "png", "bmp", "tif", "tiff", "gif", "webp", "heic",
                            "heif", "cr2", "nef", "dng", "svg", "psd", "avif",
                        ],
                    )
                    .open_single_file()
                    .show()
                    .unwrap();

                if let Some(p) = path {
                    self.input_file = p.to_string_lossy().into_owned();
                    let input = self.input_file.split(".").last().unwrap();
                    self.format_in = input.to_string();
                } else {
                    println!("No file selected.");
                }
            }

            let dir_label = ui.label("Output Directory: ");
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

            if ui.button("Convert").clicked() {
                button_sound();
                if !self.status_pending.load(Ordering::Relaxed) {
                    self.reset_download_status();
                    self.start_download_status();

                    let input = self.input_file.clone();
                    let directory = self.out_directory.clone();
                    let format_out = self.format_out.clone();
                    let complete = self.status_complete.clone();
                    let doing = self.status_pending.clone();

                    tokio::task::spawn(async move {
                        download(input, directory, format_out).await;
                        complete.store(true, Ordering::Relaxed);
                        doing.store(false, Ordering::Relaxed);
                        done_sound();
                    });
                }
            }
        });
    }
}

async fn download(input: String, directory: String, format_out: String) {
    let filename = input.split("/").last().unwrap().split(".").nth(0).unwrap();

    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&input)
        .arg("-q:v")
        .arg("100")
        .arg(format!("{}.{}", filename, format_out))
        .current_dir(directory)
        .output()
        .expect("Failed to execute command");

    let log = String::from_utf8(output.stdout).unwrap_or_else(|_| "Life suck".to_string());
    println!("{log}");
}
