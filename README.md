# NETEASE CLOUD MUSIC TUI

>A netease cloud music client for the terminal written in Rust.

![Demo](https://i.loli.net/2019/12/06/n6DCTS4cW2Z1dmH.gif)

## Dependence

This project use `gstreamer` as player. you need to install some dependence which you can find details in [gstreamer-rs](https://github.com/sdroege/gstreamer-rs)

### Linux
On Debian/Ubuntu they can be installed with
```bash
$ apt-get install libgstreamer-plugins-bad1.0-dev
```

### macOS

You can install GStreamer and the plugins via Homebrew or by installing the binaries provided by the GStreamer project.

#### Homebrew

```
$ brew install gstreamer gst-plugins-base gst-plugins-good \
      gst-plugins-bad gst-plugins-ugly gst-libav gst-rtsp-server \
      gst-editing-services --with-orc --with-libogg --with-opus \
      --with-pango --with-theora --with-libvorbis --with-libvpx \
      --enable-gtk3
```
#### GStreamer Binaries

You need to download the two .pkg files from the GStreamer website and install them, e.g. `gstreamer-1.0-1.12.3-x86_64.pkg` and `gstreamer-1.0-devel-1.12.3-x86_64.pkg`.

After installation, you also need to install pkg-config (e.g. via Homebrew) and set the PKG_CONFIG_PATH environment variable

```
$ export PKG_CONFIG_PATH="/Library/Frameworks/GStreamer.framework/Versions/Current/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
```

## Installation

### Manual
1. you can download the latest binary  `ncmt` for your OS
2. `cd` to the file you just downloaded and unzip
3. `cd` to folder and run with `./ncmt`

### Cargo

First, install [Rust](https://www.rust-lang.org/tools/install) (using the recommended `rustup` installation method) and then
```
git clone https://github.com/betta-cyber/netease-music-tui.git
cd netease-music-tui

// build release
cargo build --release
```

and then you can make some soft link to the binary and use it.

## Configuration

A configuration file is located at ${HOME}/.config/netease-music-tui/Settings.toml

The following is a sample Settings.toml file:
```toml
# Your account username (email/cellphone).
username = "username"
# Your account password.
password = "password"
```
Fill your account info to the config file.

## Usage

The binary is named ```ncmt```

When running netease-music-tui press ? to bring up a help menu that shows currently implemented key events and their actions.

This table shows some key binds

| Description | Event | Context |
| ------------- | ---------------- | --------------- |
| Increase volume | + | General |
| Decrease volume | - | General |
| Skip to next track | n | General |
| Skip to previous track | p | General |
| Seek forwards | > | General |
| Seek backwards | < | General |
| Seek backwards | < | General |
| Toggle repeat mode | r | General |
| Move selection left | h \| \<Left Arrow Key>  | General |
| Move selection down | j \| \<Down Arrow Key>  | General |
| Move selection up | k \| \<Up Arrow Key>  | General |
| Move selection right | l \| \<Right Arrow Key>  | General |
| Jump to currently playing album | a | General |
| Enter Search | / | General |
| Pause/Resume playback | \<Space> | General |
| Fullsize playbar | f | General |
| Go back or exit when nowhere left to back to | q | General |
| Enter hover mode | \<Esc>  | General |
| like current playing track | \<Ctrl+y> | General |
| dislike current playing track | \<Ctrl+d> | General |
| move track to trash | \<Ctrl+t> | Fm block |
| Enter active mode | \<Enter> | Hover mode |
| Delete entire input | \<Ctrl+u> | Search input |
| Search with input text | \<Enter>| Search input |
| Jump to start of input | \<Ctrl+a> | Search input |
| Jump to end of input | \<Ctrl+e> | Search input |
| Subscribe current hover playlist | \<Alt+s> | Playlist block |,
| Unsubscribe current hover playlist | \<Alt+d> | Playlist block |,
| Jump to next page | \<Ctrl+f> | Search result \| top list |
| Jump to previous page | \<Ctrl+b> | Search result \| top list |

## Dev plan
- [x] Djradio and djprogram
- [ ] User page
- [ ] Spectrum effect
- [ ] Comment function
- [ ] Docker deploy
- [ ] mpris support
- [ ] remove gstreamer

## License
MIT
