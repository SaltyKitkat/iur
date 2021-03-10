mod measure;

use clap::*;

enum AppMode<'a> {
    Read,
    Apply(&'a ArgMatches<'a>),
    Measure(&'a ArgMatches<'a>),
}
use AppMode::*;

struct Config {
    undervolts: [i32; 5],
    powerlimit: [i32; 2],
    tjoffset: Option<i32>,
}
impl Config {
    fn from_file() -> Option<Config> {
        todo!()
    }
}

fn main() {
    let app_m = App::new("intel-undervolt-rs")
        .subcommands(vec![
            SubCommand::with_name("read"),
            SubCommand::with_name("apply").args(&[Arg::with_name("dry-run")
                .long("dry-run")
                .short("n")
                .help("perform a trial run with no changes made")]),
            SubCommand::with_name("measure").args(&[Arg::with_name("delay-time")
                .long("delay-time")
                .short("d")
                .takes_value(true)
                .help("Specifies  the  delay between screen updates")]),
        ])
        .get_matches();

    match app_m.subcommand() {
        ("read", Some(_)) => app_run(Read),
        ("apply", Some(sub_m)) => app_run(Apply(sub_m)),
        ("measure", Some(sub_m)) => app_run(Measure(sub_m)),
        _ => println!("{}", app_m.usage()),
    }
}

fn app_run(mode: AppMode) {
    if let Measure(args) = mode {
        measure::sub_run(args);
    }
    let config = Config::from_file().expect("Failed to setup program, quiting ...");
    todo!()
}
