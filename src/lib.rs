pub mod measure;
// mod bin;
use clap::*;

enum AppMode<'a> {
    Read,
    Apply(&'a ArgMatches<'a>),
    Measure(&'a ArgMatches<'a>),
}
use AppMode::*;

pub struct Config {
    undervolts: [i32; 5],
    powerlimit: [i32; 2],
    tjoffset: Option<i32>,
}
impl Config {
    fn from_file() -> Option<Config> {
        todo!()
    }
}


fn app_run(mode: AppMode) {
    if let Measure(args) = mode {
        measure::sub_run(args);
    }
    let config = Config::from_file().expect("Failed to setup program, quiting ...");
    todo!()
}
