#[allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use serde_json::Value;

use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub alias: Option<Vec<String>>,
}
