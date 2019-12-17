#[allow(non_snake_case)]
use serde::{Serialize, Deserialize};


#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DjProgram {
    pub mainSong: i32,
    pub Songs: String,
    pub dj: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgramsRes {
    pub programs: Vec<Program>,
    pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgramDetailRes {
    pub program: Program,
    pub code: i32,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Program {
    pub mainSong: MainSong,
    pub radio: Radio,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Radio {
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
