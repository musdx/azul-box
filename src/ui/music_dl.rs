use crate::ui::shares::notify::{button_sound, done_sound, notification_done};
use eframe::egui::{self, Color32};
use native_dialog::DialogBuilder;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
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
    pub sub_lang: String,
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
            sub_lang: "en".to_string(),
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
                    egui::RichText::new(&lang).color(Color32::LIGHT_GREEN),
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
        ui.menu_button("Langs", |ui| {
            for (lang, code) in languages.iter().zip(language_codes.iter()) {
                self.sub_button(ui, code.to_string(), lang.to_string());
            }
        });
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
                    self.lang_choice(ui);
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
                button_sound();
                if !self.status_pending.load(Ordering::Relaxed) {
                    self.reset_download_status();
                    self.start_download_status();

                    let link = self.link.clone();
                    let directory = self.out_directory.clone();
                    let format = self.format.clone();
                    let complete = self.status_complete.clone();
                    let doing = self.status_pending.clone();
                    let lyrics = self.lyrics.clone();
                    let frags = self.frag.clone();
                    let lang_code = self.sub_lang.clone();

                    tokio::task::spawn(async move {
                        download(link, directory, format, lyrics, frags, lang_code).await;
                        complete.store(true, Ordering::Relaxed);
                        doing.store(false, Ordering::Relaxed);
                        done_sound();
                    });
                }
            }
        });
    }
}

async fn download(
    link: String,
    directory: String,
    format: i8,
    lyrics: bool,
    frags: i8,
    lang_code: String,
) {
    if format == 1 {
        format_dl(link, directory, "opus", lyrics, frags, lang_code).await;
    } else if format == 2 {
        format_dl(link, directory, "flac", lyrics, frags, lang_code).await;
    } else if format == 3 {
        format_dl(link, directory, "mp3", lyrics, frags, lang_code).await;
    } else if format == 4 {
        format_dl(link, directory, "m4a", lyrics, frags, lang_code).await;
    } else if format == 5 {
        format_dl(link, directory, "wav", lyrics, frags, lang_code).await;
    }
}

async fn format_dl(
    link: String,
    directory: String,
    format_name: &str,
    lyrics: bool,
    frags: i8,
    lang_code: String,
) {
    let n = frags.to_string();
    println!("{n}");

    let mut yt = Command::new("yt-dlp");
    yt.arg("--concurrent-fragments")
        .arg(&n)
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
        .current_dir(&directory);

    if lyrics {
        yt.arg("--write-subs")
            .arg("--write-auto-subs")
            .arg("--convert-subs")
            .arg("lrc")
            .arg("--exec")
            .arg("{}");

        if lang_code != "en" {
            yt.arg("--sub-langs").arg(&lang_code);
        }

        yt.arg(&link);
        let output = yt.output().expect("Failed to execute command");

        let log = String::from_utf8(output.stdout).unwrap_or_else(|_| "Life suck".to_string());
        println!("{log}");

        let regex = Regex::new(r"\[Exec\] Executing command: '(?:[^']|'')*'").unwrap();
        let files: Vec<&str> = regex.find_iter(&log).map(|file| file.as_str()).collect();

        lyrics_work(files, format_name, directory);
    } else {
        yt.arg(&link);
        let output = yt.output().expect("Failed to execute command");
        let log = String::from_utf8(output.stdout).unwrap_or_else(|_| "Life suck".to_string());
        println!("{log}");
    }
    let _ = notification_done("music downloader");
}

fn lyrics_work(files: Vec<&str>, format_name: &str, directory: String) {
    let regex = Regex::new("'[^']*'").unwrap();
    for i in files.into_iter() {
        println!("i: {i}");
        let item = regex.find(i).unwrap().as_str().trim();
        println!("item: {item}");
        let extension = format!(".{}", format_name);
        let filename = &item.split("/").last().unwrap();
        let filename = filename.split(&extension).nth(0).unwrap();
        println!("filename: {filename}");
        let music_file = &item[1..item.len() - 1].to_string();
        let lyrics_file = finder_lyrics(&directory, &filename).unwrap();
        let music_file = Path::new(&music_file);
        let lyrics = match fs::read_to_string(&lyrics_file) {
            Ok(file) => file,
            Err(error) => {
                println!("{:?}", error);
                "No-1-1!!!F".to_string()
            }
        };
        if !(lyrics == "No-1-1!!!F") && format_name == "flac" {
            let _output = Command::new("metaflac")
                .arg("--set-tag=lyrics=".to_owned() + &lyrics)
                .arg(music_file)
                .output()
                .expect("This Should Not Be IT");
            let log = String::from_utf8(_output.stdout).unwrap_or_else(|_| "Life suck".to_string());
            println!("{log}");
            let _ = fs::remove_file(&lyrics_file);
        } else if !(lyrics == "No-1-1!!!F") && format_name == "mp3" {
            todo!()
        }
    }
}

fn finder_lyrics(directory: &str, filename: &str) -> Option<PathBuf> {
    let elements = fs::read_dir(&directory).ok()?;
    let direc = PathBuf::new();
    let mut thing = Some(direc);

    for item in elements {
        let path = item.ok()?.path();
        if path.is_file() {
            if path.extension().and_then(|ext| ext.to_str()) == Some("lrc") {
                if let Some(file) = path.file_name().and_then(|name| name.to_str()) {
                    if file.contains(filename) {
                        thing = Some(path);
                    }
                }
            }
        }
    }
    thing
}
