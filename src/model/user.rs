use serde_derive::{Deserialize, Serialize};
// use serde_json::Value;

// use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub level: Option<i32>,
    pub listenSongs: Option<i32>,
    pub createDays: Option<i32>,
    pub profile: Option<Profile>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub nickname: Option<String>,
    pub gender: Option<i32>,
    pub userId: Option<i32>,
    pub followeds: Option<i32>,
    pub follows: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<i32>,
    pub userName: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Login {
    pub account: Option<Account>,
    pub profile: Option<Profile>,
    pub code: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    pub profile: Profile,
}
