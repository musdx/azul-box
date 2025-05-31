# Azul Box
<img src="./assets/logo.png" width="200"></img>

## Feature

- Download music from link with metadata/cover embedded with synced lyric(flac only)
- Download Video from link with highest quality with some metadata embeded
- Download Pin from pinterest without account
- Convert Images formats powered by ffmpeg
- Convert Video formats powered by ffmpeg
- A trash color picker

The video/music download will technically support all yt-dlp [support list](https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md) not just youtube.

The features will be really random because this is a software I made to randomly do something I needed.

## Dependencies

- ffmpeg & ffprobe
- yt-dlp
- mutagen
- metaflac(from the flac package)

### Archlinux:

```
sudo pacman -S ffmpeg yt-dlp python-mutagen flac
```

### Linuxmint:

```
sudo apt install ffmpeg yt-dlp python3-mutagen flac
```

### openSUSE Tumbleweed:

You may need to add some extra official opensuse repo.

```
sudo zypper in yt-dlp python-mutagen flac
```

### Fedora(non atomic):

```
sudo dnf install ffmpeg-free yt-dlp python3-mutagen flac
```

## Installation

You will need to have [rustup](https://rustup.rs/) already installed.

Use this follow command to build then install the app. You will still need to install dependencies before run the install script.

#### You may want to use download the release source and build from there instead of clone the repo.
```
git clone https://github.com/musdx/azul-box
cd azul-box
sudo chmod +x install.sh
./install.sh
```

## Showcase

<div align="center">
<img src="./assets/pic1.png" width="450"></img>
<img src="./assets/pic2.png" width="450"></img>
</div>

[azul-0.1.0.webm](https://github.com/user-attachments/assets/aa9c66fa-2fcf-4e7f-a454-e2b73fdf8f19)
