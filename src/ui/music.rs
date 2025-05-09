use eframe::egui::{self, Color32};
use native_dialog::DialogBuilder;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::process::Command;

pub struct MusicDownload {
    pub link: String,
    pub out_directory: String,
    pub status_complete: Arc<AtomicBool>,
    pub status_pending: Arc<AtomicBool>,
    pub format: i8,
}

impl Default for MusicDownload {
    fn default() -> Self {
        let default_directory = dirs::audio_dir()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| String::from(""));
        Self {
            link: String::new(),
            out_directory: default_directory,
            status_complete: Arc::new(AtomicBool::new(false)),
            status_pending: Arc::new(AtomicBool::new(false)),
            format: 1,
        }
    }
}

impl MusicDownload {
    fn reset_download_status(&mut self) {
        self.status_complete.store(false, Ordering::Relaxed);
        self.status_pending.store(false, Ordering::Relaxed);
    }
    fn start_download_status(&mut self) {
        self.status_pending.store(true, Ordering::Relaxed);
    }
    fn format_button(&mut self, ui: &mut egui::Ui, name: &str, numbername: i8) {
        if self.format == numbername {
            if ui
                .add(egui::Button::new(
                    egui::RichText::new(name).color(Color32::LIGHT_GREEN),
                ))
                .clicked()
            {
                self.format = numbername;
            };
        } else {
            if ui.button(name).clicked() {
                self.format = numbername;
            };
        }
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("Setting", |ui| {
                ui.menu_button("Format", |ui| {
                    self.format_button(ui, "OPUS", 1);
                    self.format_button(ui, "FLAC", 2);
                    self.format_button(ui, "MP3", 3);
                    self.format_button(ui, "M4A", 4);
                    self.format_button(ui, "WAV", 5);
                });
                if ui.button("Close").clicked() {
                    ui.close_menu();
                }
            });
            ui.label("Status: ");
            if self.status_complete.load(Ordering::Relaxed) {
                ui.colored_label(Color32::GREEN, "Done!");
            } else if self.status_pending.load(Ordering::Relaxed) {
                ui.spinner();
            }
        });
        ui.separator();
        ui.vertical_centered(|ui| {
            let link_label = ui.label("link: ");
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
                self.reset_download_status();
                self.start_download_status();

                let link = self.link.clone();
                let directory = self.out_directory.clone();
                let format = self.format.clone();
                let complete = self.status_complete.clone();
                let doing = self.status_pending.clone();

                tokio::task::spawn(async move {
                    download(link, directory, format).await;
                    complete.store(true, Ordering::Relaxed);
                    doing.store(false, Ordering::Relaxed);
                });
            }
        });
    }
}

async fn download(link: String, directory: String, format: i8) {
    if format == 1 {
        format_dl(link, directory, "opus").await;
    } else if format == 2 {
        format_dl(link, directory, "flac").await;
    } else if format == 3 {
        format_dl(link, directory, "mp3").await;
    } else if format == 4 {
        format_dl(link, directory, "m4a").await;
    } else if format == 5 {
        format_dl(link, directory, "wav").await;
    }
}
async fn format_dl(link: String, directory: String, format_name: &str) {
    let output = Command::new("yt-dlp")
        .arg("-i")
        .arg("-x")
        .arg("--audio-quality")
        .arg("0")
        .arg("--audio-format")
        .arg(format_name)
        .arg("--embed-thumbnail")
        .arg("--add-metadata")
        .arg("--metadata-from-title")
        .arg("%(title)s")
        .arg("--parse-metadata")
        .arg("title:%(title)s")
        .arg("--parse-metadata")
        .arg("uploader:%(artist)s")
        .arg("--output")
        .arg("%(title)s.%(ext)s")
        .arg(link)
        .current_dir(directory)
        .output()
        .await
        .expect("Failed to execute command");

    println!("{:?}", output)
}
