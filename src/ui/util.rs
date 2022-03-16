extern crate chrono;
use super::super::model::artist::Artist;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{Duration, UNIX_EPOCH};
use tui::style::{Color, Style};
use regex::Regex;
use config::Config;

const FILE_NAME: &str = "Settings.toml";
const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "netease-music-tui";

lazy_static! {
    /**
     * Regex matches:
     * - group 1: Red
     * - group 2: Green
     * - group 3: Blue
     */
    static ref COLOR_HEX_REGEX: Regex = Regex::new(r"#(:?[0-9a-fA-F]{2})(:?[0-9a-fA-F]{2})(:?[0-9a-fA-F]{2})").unwrap();

    static ref SETTINGS: Config = Config::builder()
        .add_source(config::File::with_name(
                &format!("{}/{}/{}/{}", &dirs::home_dir().unwrap().to_string_lossy(), CONFIG_DIR, APP_CONFIG_DIR, FILE_NAME)))
        .build()
        .unwrap();
}

pub fn parse_hex_color(color: &str) -> Option<Color> {
    COLOR_HEX_REGEX.captures(color).map(|groups| {
        Color::Rgb(
            u8::from_str_radix(groups.get(1).unwrap().as_str(), 16)
                .ok()
                .unwrap(),
            u8::from_str_radix(groups.get(2).unwrap().as_str(), 16)
                .ok()
                .unwrap(),
            u8::from_str_radix(groups.get(3).unwrap().as_str(), 16)
                .ok()
                .unwrap(),
        )
    })
}


pub fn get_color((is_active, is_hovered): (bool, bool)) -> Style {
    match (is_active, is_hovered) {
        (true, _) => match SETTINGS.get_string("active") {
            Ok(s) => Style::default().fg(parse_hex_color(&s).unwrap()),
            Err(_) => Style::default().fg(Color::LightCyan),
        }
        (false, true) => match SETTINGS.get_string("hover") {
            Ok(s) => Style::default().fg(parse_hex_color(&s).unwrap()),
            Err(_) => Style::default().fg(Color::Magenta),
        }
        _ => match SETTINGS.get_string("other") {
            Ok(s) => Style::default().fg(parse_hex_color(&s).unwrap()),
            Err(_) => Style::default().fg(Color::Gray),
        }
    }
}

pub fn get_text_color() -> Style {
    match SETTINGS.get_string("text") {
        Ok(s) => Style::default().fg(parse_hex_color(&s).unwrap()),
        Err(_) => Style::default().fg(Color::Gray),
    }
}

// `percentage` param needs to be between 0 and 1
pub fn get_percentage_width(width: u16, percentage: f32) -> u16 {
    let padding = 3;
    let width = width - padding;
    (f32::from(width) * percentage) as u16
}

pub fn millis_to_minutes(millis: u64) -> String {
    let minutes = millis / 60000;
    let seconds = (millis % 60000) / 1000;
    let seconds_display = if seconds < 10 {
        format!("0{}", seconds)
    } else {
        format!("{}", seconds)
    };

    if seconds == 60 {
        format!("{}:00", minutes + 1)
    } else {
        format!("{}:{}", minutes, seconds_display)
    }
}

// display track progress for progress bar
pub fn display_track_progress(progress: u64, track_duration: u64) -> String {
    let duration = millis_to_minutes(u64::from(track_duration));
    let progress_display = millis_to_minutes(progress);
    let remaining = millis_to_minutes(u64::from(track_duration) - progress);

    format!("{}/{} (-{})", progress_display, duration, remaining,)
}

pub fn create_artist_string(artists: &[Artist]) -> String {
    artists
        .iter()
        .map(|artist| artist.name.to_string())
        .collect::<Vec<String>>()
        .join("/ ")
}

pub fn create_tag_string(tags: &[String]) -> String {
    tags.iter()
        .map(|tag| tag.to_string())
        .collect::<Vec<String>>()
        .join("|")
}

pub fn create_datetime_string(timestamp: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_millis(timestamp);
    let datetime = DateTime::<Utc>::from(d);
    datetime.format("%Y-%m-%d").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_datatime_string() {
        assert_eq!(create_datetime_string(1576472814620), "2019-12-16");
    }
}
