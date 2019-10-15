use serde::{Serialize, Deserialize};
use serde_json::Value;

use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub level: Option<i32>,
    pub listenSongs: Option<i32>,
    pub createDays: Option<i32>,
    pub profile: Option<Profile>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub nickname: Option<String>,
    pub gender: Option<i32>,
    pub userId: Option<i32>,
    pub followeds: Option<i32>,
    pub follows: Option<i32>
}
