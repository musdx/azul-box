use tokio::process::Command;

pub async fn download(link: String, directory: String) {
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
