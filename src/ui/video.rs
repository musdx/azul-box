use eframe::egui::{self, Color32};
use native_dialog::DialogBuilder;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::process::Command;

pub struct VideoDownload {
    pub link: String,
    pub out_directory: String,
    pub status_complete: Arc<AtomicBool>,
    pub status_pending: Arc<AtomicBool>,
    pub format: i8,
}

impl Default for VideoDownload {
    fn default() -> Self {
        let default_directory = dirs::video_dir()
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

impl VideoDownload {
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
                ui.close_menu();
            };
        } else {
            if ui.button(name).clicked() {
                self.format = numbername;
                ui.close_menu();
            };
        }
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("Setting", |ui| {
                ui.menu_button("Format", |ui| {
                    self.format_button(ui, "MKV", 1);
                    self.format_button(ui, "MP4", 2);
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
        mkv_dl(link, directory).await;
    } else if format == 2 {
        mp4_dl(link, directory).await;
    }
}
async fn mkv_dl(link: String, directory: String) {
    let output = Command::new("yt-dlp")
        .arg("-f")
        .arg("bestvideo+bestaudio")
        .arg("--embed-thumbnail")
        .arg("--embed-subs")
        .arg("--embed-metadata")
        .arg(link)
        .current_dir(directory)
        .output()
        .await
        .expect("Failed to execute command");

    println!("{:?}", output);
    println!("best");
}

async fn mp4_dl(link: String, directory: String) {
    let output = Command::new("yt-dlp")
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best")
        .arg("--embed-thumbnail")
        .arg("--embed-subs")
        .arg("--embed-metadata")
        .arg(link)
        .current_dir(directory)
        .output()
        .await
        .expect("Failed to execute command");

    println!("{:?}", output);
    println!("mp4");
}
