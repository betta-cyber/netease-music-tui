#[allow(non_snake_case)]
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgramsRes {
    pub programs: Vec<DjProgram>,
    pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgramDetailRes {
    pub program: DjProgram,
    pub code: i32,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubDjRadioRes {
    pub djRadios: Vec<DjRadio>,
    pub code: i32,
}

// dj program which can listen
#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DjProgram {
    pub mainSong: MainSong,
    pub radio: DjRadio,
    pub id: usize,
    // pub description: String,
    pub serialNum: usize,
    pub listenerCount: usize,
    pub createTime: u64,
}

// dj radio means dj's radio list
// not listen Program
#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DjRadio {
    pub category: String,
    pub subCount: usize,
    pub programCount: usize,
    pub desc: String,
    pub name: String,
    pub id: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MainSong {
    pub name: String,
    pub id: usize,
    pub duration: usize,
}
