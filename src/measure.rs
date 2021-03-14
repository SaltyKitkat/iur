use std::{thread::sleep, time::Duration};

use clap::ArgMatches;
mod powercap;
mod coretemp;

pub fn sub_run(args: &ArgMatches) -> ! {
    println!("measure mode");
    let delay_time = args
        .value_of("delay-time")
        .unwrap_or_default()
        .parse()
        .unwrap_or(2.0);

    let hwmon = coretemp::init();
    let mut powercap = powercap::init();
    sleep(Duration::from_millis(5));

    print!("\x1b[H\x1b[J");
    loop {
        powercap::print(&mut powercap);
        println!("\x1b[K");
        coretemp::print(&hwmon);
        println!("{}", delay_time);
        print!("\x1b[J\x1b[H");
        sleep(Duration::from_secs_f64(delay_time));
    }
}

