pub struct Config {
    undervolts: [i32; 5],
    powerlimit: [i32; 2],
    tjoffset: Option<i32>,
}
impl Config {
    pub fn from_file() -> Option<Config> {
        todo!()
    }
}
