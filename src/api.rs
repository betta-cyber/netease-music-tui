use serde_json;
use serde_json::Value;
use serde::{Serialize, Deserialize};

use reqwest::Client;
use reqwest::Method;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap};
use reqwest::StatusCode;

use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use std::string::{String, ToString};
use std::fmt::Debug;
use std::hash::Hash;
use std::borrow::Cow;
use std::time::Duration;

use super::model::user::{User, Profile, Login, Status};
use super::model::song::{Song, Songs};
use super::model::search::{SearchTrackResult, SearchPlaylistResult, SearchPlaylists, SearchTracks};
use super::model::playlist::{PlaylistRes, Playlist, Track, PlaylistDetailRes, PlaylistDetail, PersonalFmRes};

lazy_static! {
    /// HTTP Client
    pub static ref CLIENT: Client = Client::builder()
        .gzip(true)
        .cookie_store(true)
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
}

#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    RateLimited(Option<usize>),
    Other(u16)
}
impl failure::Fail for ApiError {}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Spotify API reported an error")
    }
}
impl From<&reqwest::Response> for ApiError {
    fn from(response: &reqwest::Response) -> Self {
        match response.status() {
            StatusCode::UNAUTHORIZED => ApiError::Unauthorized,
            StatusCode::TOO_MANY_REQUESTS => {
                if let Ok(duration) = response.headers()[reqwest::header::RETRY_AFTER].to_str() {
                    ApiError::RateLimited(duration.parse::<usize>().ok())
                }
                else {
                    ApiError::RateLimited(None)
                }
            },
            status => ApiError::Other(status.as_u16())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudMusic {
    pub prefix: String
}

impl CloudMusic {

    pub fn default() -> CloudMusic {
        CloudMusic {
            prefix: "http://10.1.78.190:3000".to_owned(),
        }
    }

    ///send get request
    fn get(&self, url: &str, params: &mut HashMap<String, String>) -> Result<String, failure::Error> {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.internal_call(Method::GET, &url_with_params, None)
        } else {
            self.internal_call(Method::GET, url, None)
        }
    }

    fn internal_call(&self, method: Method, url: &str, payload: Option<&Value>) -> Result<String, failure::Error> {
        let mut url: Cow<str> = url.into();
        if !url.starts_with("http") {
            url = ["http://127.0.0.1:3000", &url].concat().into();
        }

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let mut response = {
            let builder = CLIENT
                .request(method, &url.into_owned())
                .headers(headers);

            // only add body if necessary
            // spotify rejects GET requests that have a body with a 400 response
            let builder = if let Some(json) = payload {
                builder.json(json)
            } else {
                builder
            };

            builder.send().unwrap()
        };

        let mut buf = String::new();
        response
            .read_to_string(&mut buf)
            .expect("failed to read response");
        if response.status().is_success() {
            Ok(buf)
        } else if response.status() == 403 {
            Ok(buf)
        } else {
            Err(failure::Error::from(ApiError::from(&response)))
        }
    }

    pub fn login(&self, email: &str, password: &str) -> Result<Profile, failure::Error> {
        let url = format!("/login");
        let mut params = HashMap::new();
        params.insert("email".to_owned(), email.to_string());
        params.insert("password".to_owned(), password.to_string());

        let result = self.get(&url, &mut params)?;
        let login = self.convert_result::<Login>(&result).unwrap();
        Ok(login.profile.unwrap())
    }

    pub fn status(&self) -> Result<Profile, failure::Error> {
        let url = format!("/login/status");

        match self.get(&url, &mut HashMap::new()) {
            Ok(r) => {
                let login = self.convert_result::<Status>(&r).unwrap();
                Ok(login.profile.clone())
            },
            Err(e) => {
                Err(format_err!("need login"))
            }
        }
    }

    pub fn user(&self, user_id: &str) -> Result<User, failure::Error> {
        let url = format!("/user/detail");
        // url.push_str(&trid);
        let mut params = HashMap::new();
        params.insert("uid".to_owned(), user_id.to_string());

        let result = self.get(&url, &mut params)?;
        self.convert_result::<User>(&result)
    }

    pub fn get_song_url(&self, song_id: &str) -> Result<Song, failure::Error> {
        let url = format!("/song/url");
        let mut params = HashMap::new();
        params.insert("id".to_owned(), song_id.to_string());

        // send request
        let result = self.get(&url, &mut params)?;
        let songs = self.convert_result::<Songs>(&result).unwrap();
        Ok(songs.data[0].clone())
    }

    pub fn user_playlist(&self, user_id: &str) -> Result<Vec<Playlist>, failure::Error> {
        let url = format!("/user/playlist");
        let mut params = HashMap::new();
        params.insert("uid".to_owned(), user_id.to_string());

        // send request
        let result = self.get(&url, &mut params)?;
        let res = self.convert_result::<PlaylistRes>(&result).unwrap();
        Ok(res.playlist.clone())
    }

    pub fn playlist_detail(&self, playlist_id: &str) -> Result<PlaylistDetail, failure::Error> {
        let url = format!("/playlist/detail");
        let mut params = HashMap::new();
        params.insert("id".to_owned(), playlist_id.to_string());

        // send request
        let result = self.get(&url, &mut params)?;
        let res = self.convert_result::<PlaylistDetailRes>(&result).unwrap();
        Ok(res.playlist.unwrap().clone())
    }

    // search api
    pub fn search(&self, keyword: &str, search_type: &str, limit: i32, offset: i32) -> Result<String, failure::Error> {
        let url = format!("/search");
        let mut params = HashMap::new();
        params.insert("keywords".to_owned(), keyword.to_string());
        params.insert("type".to_owned(), search_type.to_string());
        params.insert("limit".to_owned(), limit.to_string());
        params.insert("offset".to_owned(), offset.to_string());

        // send request
        self.get(&url, &mut params)
    }

    // search for track
    pub fn search_track(&self, keyword: &str, limit: i32, offset: i32) -> Result<SearchTracks, failure::Error> {
        let result = self.search(keyword, "1", limit, offset)?;
        let res = self.convert_result::<SearchTrackResult>(&result)?;
        Ok(res.result.unwrap())
    }

    // search for playlist
    pub fn search_playlist(&self, keyword: &str, limit: i32, offset: i32) -> Result<SearchPlaylists, failure::Error> {
        let result = self.search(keyword, "1000", limit, offset)?;
        let res = self.convert_result::<SearchPlaylistResult>(&result)?;
        Ok(res.result.unwrap())
    }

    // get user personal fm
    pub fn personal_fm(&self, keyword: &str, limit: i32, offset: i32) -> Result<Vec<Track>, failure::Error> {
        let url = format!("/personal_fm");
        let mut params = HashMap::new();

        // send request
        let result = self.get(&url, &mut params)?;
        let res = self.convert_result::<PersonalFmRes>(&result).unwrap();
        Ok(res.data)
    }

    // get current user playlist
    pub fn user_playlists(&self, uid: &str) -> Result<Vec<Playlist>, failure::Error> {
        let url = format!("/user/playlist");
        let mut params = HashMap::new();
        params.insert("uid".to_owned(), uid.to_string());

        // send request
        let result = self.get(&url, &mut params)?;
        let res = self.convert_result::<PlaylistRes>(&result).unwrap();
        Ok(res.playlist.clone())
    }

    pub fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> Result<T, failure::Error> {
        let result = serde_json::from_str::<T>(input)
            .map_err(|e| format_err!("convert result failed, reason: {:?}; content: [{:?}]", e,input))?;
        Ok(result)
    }
}

pub fn convert_map_to_string<K: Debug + Eq + Hash+ ToString,
V: Debug+ToString>(map: &HashMap<K, V>) -> String{
    let mut string: String = String::new();
    for (key, value) in map.iter() {
        string.push_str(&key.to_string());
        string.push_str("=");
        string.push_str(&value.to_string());
        string.push_str("&");
    }
    string
}
