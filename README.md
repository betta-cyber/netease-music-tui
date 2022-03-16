# NETEASE CLOUD MUSIC TUI

[![Actions Status](https://github.com/betta-cyber/netease-music-tui/workflows/Continuous%20Integration/badge.svg)](https://github.com/betta-cyber/netease-music-tui/actions)

>A netease cloud music client for the terminal written in Rust.

![Demo](https://i.loli.net/2019/12/06/n6DCTS4cW2Z1dmH.gif)

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

// build release and install
make && make install
```

and then you can make some soft link to the binary and use it.

### Arch Linux

for Arch based distributions, users can install from [AUR](https://aur.archlinux.org/packages/netease-music-tui)
```
// build and install from source
yay -S netease-music-tui 

// install from release binary file
yay -S netease-music-tui-bin 
```

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

## custom theme

user can custom some theme color in config file. such as:

```
hover = "#565656"
active = "#abe047"
other = "#eb4129"
```

hover means hover block border color
active means current select block border color
other means other block border color

text color will add in the future.

## Dev plan
- [x] Djradio and djprogram
- [ ] User page
- [ ] Spectrum effect
- [ ] Comment function
- [ ] mpris support
- [x] remove gstreamer (but current player no seek function)

## Features

dbus mpris
`cargo run --features dbus_mpris`

## License
MIT
