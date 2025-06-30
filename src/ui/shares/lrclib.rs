use std::error::Error;
use std::path::Path;

use lofty::config::WriteOptions;
use lofty::prelude::*;
use lofty::probe::Probe;
use lofty::tag::Tag;
use serde::Deserialize;
#[allow(dead_code)]
pub fn lrclib_fetch(opt: &Path) {
    let mut tagged_file = Probe::open(&opt)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let tag = match tagged_file.primary_tag_mut() {
        Some(primary_tag) => primary_tag,
        None => {
            if let Some(first_tag) = tagged_file.first_tag_mut() {
                first_tag
            } else {
                let tag_type = tagged_file.primary_tag_type();

                eprintln!("WARN: No tags found, creating a new tag of type `{tag_type:?}`");
                tagged_file.insert_tag(Tag::new(tag_type));

                tagged_file.primary_tag_mut().unwrap()
            }
        }
    };
    let artist = tag.artist().unwrap();
    let title = tag.title().unwrap();
    let title = title.to_string().replace(" ", "+");
    let artist = artist.to_string().replace(" ", "+");
    println!("{title}\n{artist}");
    let query = format!(
        "https://lrclib.net/api/get?artist_name={}&track_name={}",
        artist, title
    );
    let lyrics = fetch(&query);
    println!("{}", query);
    match lyrics {
        Ok(ly) => {
            println!("{ly}");
            let lyric_after = translate("en", &ly);
            if lyric_after.is_ok() {
                tag.insert_text(ItemKey::Lyrics, lyric_after.unwrap());
                tag.save_to_path(opt, WriteOptions::default())
                    .expect("ERROR: Failed to write the tag!");
            }
        }
        Err(e) => {
            println!("{e}")
        }
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    plainLyrics: String,
    syncedLyrics: String,
}

fn fetch(query: &str) -> Result<String, Box<dyn Error>> {
    let resp = ureq::get(query)
        .header("User-Agent", "Azulbox (https://github.com/musdx/azul-box)")
        .call()?
        .body_mut()
        .read_json::<ApiResponse>()?;
    if !resp.syncedLyrics.is_empty() {
        Ok(resp.syncedLyrics)
    } else if !resp.plainLyrics.is_empty() {
        Ok(resp.plainLyrics)
    } else {
        Ok(String::new())
    }
}

use serde_json::Value;
use url::form_urlencoded;

fn translate(to: &str, text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let en_text: String = form_urlencoded::byte_serialize(text.as_bytes()).collect();
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl={}&dt=t&q={}",
        to, en_text
    );

    let mut translated_text = text.to_string();

    let response = ureq::get(&url).call();
    match response {
        Ok(mut re) => {
            let string_body = re.body_mut().read_to_string();

            if string_body.is_ok() {
                let json_as_string = &string_body.ok().unwrap();
                let values = serde_json::from_str::<Value>(json_as_string)?;
                if let Some(value) = values.get(0) {
                    println!("{:?}", value.as_str());
                    if let Some(list) = value.as_array() {
                        let lyrics: Vec<String> = list
                            .iter()
                            .filter_map(|v| v.get(0).and_then(|v| v.as_str()))
                            .map(|s| s.to_string())
                            .collect();
                        translated_text = lyrics.join("");
                        println!("Translate success!");
                    }
                }
            }
        }
        Err(e) => {
            println!("{e}");
        }
    }
    Ok(translated_text)
}
