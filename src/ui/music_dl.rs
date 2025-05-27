use eframe::egui::{self, Color32};
use native_dialog::DialogBuilder;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct MusicDownload {
    pub link: String,
    pub out_directory: String,
    pub status_complete: Arc<AtomicBool>,
    pub status_pending: Arc<AtomicBool>,
    pub format: i8,
    pub lyrics: bool,
    pub frag: i8,
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
            format: 2,
            lyrics: false,
            frag: 1,
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
        if !(self.format == 2) {
            self.lyrics = false;
        }
        ui.horizontal(|ui| {
            ui.menu_button("Setting", |ui| {
                ui.menu_button("Format", |ui| {
                    self.format_button(ui, "OPUS", 1);
                    self.format_button(ui, "FLAC", 2);
                    self.format_button(ui, "MP3", 3);
                    self.format_button(ui, "M4A", 4);
                    self.format_button(ui, "WAV", 5);
                });
                if self.lyrics && self.format == 2 {
                    if ui
                        .add(egui::Button::new(
                            egui::RichText::new("Lyrics").color(Color32::LIGHT_GREEN),
                        ))
                        .clicked()
                    {
                        self.lyrics = false;
                    };
                } else if self.format == 2 {
                    if ui.button("Lyrics").clicked() {
                        self.lyrics = true;
                    };
                }
                ui.add(egui::widgets::Slider::new(&mut self.frag, 1..=10).text("Fragments"));
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
                let lyrics = self.lyrics.clone();
                let frags = self.frag.clone();

                tokio::task::spawn(async move {
                    download(link, directory, format, lyrics, frags).await;
                    complete.store(true, Ordering::Relaxed);
                    doing.store(false, Ordering::Relaxed);
                });
            }
        });
    }
}

async fn download(link: String, directory: String, format: i8, lyrics: bool, frags: i8) {
    if format == 1 {
        format_dl(link, directory, "opus", lyrics, frags).await;
    } else if format == 2 {
        format_dl(link, directory, "flac", lyrics, frags).await;
    } else if format == 3 {
        format_dl(link, directory, "mp3", lyrics, frags).await;
    } else if format == 4 {
        format_dl(link, directory, "m4a", lyrics, frags).await;
    } else if format == 5 {
        format_dl(link, directory, "wav", lyrics, frags).await;
    }
}
async fn format_dl(link: String, directory: String, format_name: &str, lyrics: bool, frags: i8) {
    let n = frags.to_string().to_owned();
    println!("{n}");
    if lyrics {
        let output = Command::new("yt-dlp")
            .arg("--concurrent-fragments")
            .arg(n)
            .arg("-i")
            .arg("-x")
            .arg("--audio-quality")
            .arg("0")
            .arg("--audio-format")
            .arg(&format_name)
            .arg("--write-subs")
            .arg("--convert-subs")
            .arg("srt")
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
            .arg("--exec")
            .arg("{}")
            .arg(&link)
            .current_dir(&directory)
            .output()
            .expect("Failed to execute command");

        let log = String::from_utf8(output.stdout).unwrap_or("Life suck".to_string());
        println!("{log}");

        let regex = Regex::new(r"\[Exec\] Executing command: '(?:[^']|'')*'").unwrap();
        let files: Vec<&str> = regex.find_iter(&log).map(|file| file.as_str()).collect();

        lyrics_work(files, format_name, directory);
    } else {
        let output = Command::new("yt-dlp")
            .arg("--concurrent-fragments")
            .arg(n)
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
            .expect("Failed to execute command");

        println!("{:?}", output)
    }
}

fn lyrics_work(files: Vec<&str>, format_name: &str, directory: String) {
    println!("I run");
    let regex = Regex::new("'[^']*'").unwrap();
    for i in files.into_iter() {
        println!("i: {i}");
        let item = regex.find(i).unwrap().as_str().trim();
        println!("item: {item}");
        let filename = &item.split(format_name).nth(0).unwrap().replace("'", "");
        let filename = filename.split("/").last().unwrap();
        println!("filename: {filename}");
        let lyrics_file = format!("{}/{}en.srt", &directory, &filename);
        let music_file = format!("{}/{}{}", &directory, &filename, &format_name);
        let lyrics_file = Path::new(&lyrics_file);
        let music_file = Path::new(&music_file);
        let lyrics = match fs::read_to_string(lyrics_file) {
            Ok(file) => file,
            Err(error) => {
                println!("{:?}", error);
                "No-1-1!!!F".to_string()
            }
        };
        if !(lyrics == "No-1-1!!!F") {
            let _output = Command::new("metaflac")
                .arg("--set-tag=lyrics=".to_owned() + &lyrics)
                .arg(music_file)
                .output();
            println!("{:?}", _output);
            let _ = fs::remove_file(&lyrics_file);
        };
    }
}
