use std::{path::Path, process::exit};

use clap::{App, Arg, ArgMatches, SubCommand};
use iur::{modes::*, Config};

enum IurMode<'a> {
    Read,
    Apply(&'a ArgMatches<'a>),
    Measure(&'a ArgMatches<'a>),
}
use IurMode::*;

fn app_run(mode: IurMode) -> ! {
    match mode {
        Measure(args) => measure::sub_run(args),
        Apply(args) => {
            // it has default_value("/etc/iur/iur.conf") so it should be safe to unwrap here.
            let config_path = Path::new(match args.value_of("with-config") {
                Some(v) => v,
                None => {
                    // panic!("config file default option failed!!!\n");
                    unreachable!()
                }
            });
            match Config::from_file(config_path) {
                Some(conf) => {
                    if args.occurrences_of("dry-run") != 0 {
                        println!("{}", conf);
                        exit(0);
                    }
                    rw::apply(conf);
                }
                None => panic!(
                    "Failed to load config file from {}",
                    config_path.as_os_str().to_string_lossy()
                ),
            };
        }
        Read => rw::read(),
    }
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
        ("read", Some(_)) => app_run(Read),
        ("apply", Some(sub_m)) => app_run(Apply(sub_m)),
        ("measure", Some(sub_m)) => app_run(Measure(sub_m)),
        _ => println!("{}", app_m.usage()),
    }
}
