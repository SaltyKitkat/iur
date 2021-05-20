use std::path::Path;

pub struct Config {
    undervolts: [i32; 5],
    powerlimit: [i32; 2],
    tjoffset: Option<i32>,
}
impl Config {
    pub fn from_file(config_path: &Path) -> Option<Config> {
        todo!()
    }

    pub fn from_raw() -> Option<Config> {
        todo!()
    }
}
impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
