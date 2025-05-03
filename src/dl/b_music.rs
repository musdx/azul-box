use tokio::process::Command;

pub async fn download(link: String, directory: String) {
    let output = Command::new("yt-dlp")
        .arg("-i")
        .arg("-x")
        .arg("--audio-quality")
        .arg("0")
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
