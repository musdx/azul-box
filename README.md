

# Azul Box

<img src="./assets/logo.png" width="200"></img>

<noscript><a href="https://liberapay.com/musdx/donate"><img alt="Donate using Liberapay" src="https://liberapay.com/assets/widgets/donate.svg"></a></noscript>

## Feature

- Download music from link with metadata/cover embedded with synced lyric(lyric do not work for WAV)
- Download Video from link with highest quality with some metadata embeded
- Download Pin from pinterest without account
- Convert Images formats powered by ffmpeg
- Convert Video formats powered by ffmpeg

The video/music download will technically support all yt-dlp [support list](https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md) not just youtube.

The features will be really random because this is a software I made to randomly do something I needed.

## Dependencies

- [rustup](https://rustup.rs/)
- ffmpeg & ffprobe
- yt-dlp
- mutagen

### Archlinux:

```
sudo pacman -S ffmpeg yt-dlp python-mutagen
```

### Linuxmint:

```
sudo apt install ffmpeg yt-dlp python3-mutagen
```

### openSUSE Tumbleweed:

You may need to add some extra official opensuse repo.

```
sudo zypper in yt-dlp python-mutagen
```

### Fedora(non atomic):

```
sudo dnf install ffmpeg-free yt-dlp python3-mutagen
```

## Installation

Use this follow command to build then install the app. You will still need to install dependencies before run the install script.

### Debian based distro

- Download the deb file from [release](https://github.com/musdx/azul-box/releases)
- Install it via apt
- Enjoy!!!

### Any other distro

- Download the bin.zip file from [release](https://github.com/musdx/azul-box/releases)
- Unzip it
- Run the install.sh file init
- Enjoy!!!

## Uninstallation

```
sudo rm /usr/bin/azulbox
sudo rm /usr/share/icons/azul_box.png
sudo rm /usr/share/applications/azul_box.desktop
```

or just uninstall via your package manager if you install via apt

## Showcase

<div align="center">
<img src="./assets/pic1.png" width="450"></img>
<img src="./assets/pic2.png" width="450"></img>
</div>

[v0.1.6.webm](https://github.com/user-attachments/assets/390744b3-a4df-488e-8091-cd92455b69c1)
