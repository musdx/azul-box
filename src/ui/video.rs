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
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("Setting", |ui| {
                ui.menu_button("Format", |ui| {
                    ui.label("still empty");
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
                let complete = self.status_complete.clone();
                let doing = self.status_pending.clone();

                tokio::task::spawn(async move {
                    download(link, directory).await;
                    complete.store(true, Ordering::Relaxed);
                    doing.store(false, Ordering::Relaxed);
                });
            }
        });
    }
}

async fn download(link: String, directory: String) {
    let output = Command::new("yt-dlp")
        .arg("-f")
        .arg("bestvideo+bestaudio")
        .arg("--embed-thumbnail")
        .arg("--embed-subs")
        .arg("--add-metadata")
        .arg("--metadata-from-title")
        .arg("--write-info-json")
        .arg(link)
        .current_dir(directory)
        .output()
        .await
        .expect("Failed to execute command");

    println!("{:?}", output);
}
