#[derive(Debug)]
pub struct Error {
    pub msg: String,
    pub code: i32,
}

pub const GEN_ERROR: i32 = 12;

impl Error {
    pub fn new(msg: &str, code: i32) -> Self {
        Self {
            msg: msg.to_string(),
            code,
        }
    }
}
