use clap::{App, Arg, ArgMatches, SubCommand};
use iur::measure;
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

enum IurMode<'a> {
    Read,
    Apply(&'a ArgMatches<'a>),
    Measure(&'a ArgMatches<'a>),
}
fn app_run(mode: IurMode) {
    if let IurMode::Measure(args) = mode {
        measure::sub_run(args);
    }
    let config = Config::from_file().expect("Failed to setup program, quiting ...");
    todo!()
}
fn main() {
    let app_m = App::new("intel-undervolt-rs")
        .bin_name("iur")
        .subcommands(vec![
            SubCommand::with_name("read"),
            SubCommand::with_name("apply").args(&[
                Arg::with_name("dry-run")
                    .long("dry-run")
                    .short("n")
                    .help("perform a trial run with no changes made"),
                Arg::with_name("with-config")
                    .long("with-config")
                    .short("c")
                    .takes_value(true)
                    .default_value("/etc/iur/iur.conf"),
            ]),
            SubCommand::with_name("measure").args(&[Arg::with_name("delay-time")
                .long("delay-time")
                .short("d")
                .takes_value(true)
                .help("Specifies  the  delay between screen updates")]),
        ])
        .get_matches();

    match app_m.subcommand() {
        ("read", Some(_)) => app_run(IurMode::Read),
        ("apply", Some(sub_m)) => app_run(IurMode::Apply(sub_m)),
        ("measure", Some(sub_m)) => app_run(IurMode::Measure(sub_m)),
        _ => println!("{}", app_m.usage()),
    }
}
