use std::fmt;

pub enum Status {
    Loading,
    Processing,
    Ready
}

pub struct State {
    pub status: Status,
    pub file_path: String,
    pub split_ceil: f32,
    pub data: Vec<i16>,
    pub splits: Vec<u32>,
}

impl State {
    pub fn new() -> State {
        State {
            status: Status::Ready,
            file_path: String::new(),
            split_ceil: 1.0,
            data: vec![],
            splits: vec![],
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State {{ file_path: {}, split_ceil: {} }}", 
               self.file_path, 
               self.split_ceil)
    }
}

