use std::{thread::sleep, time::Duration};

use clap::ArgMatches;
mod coretemp;
mod powercap;
use powercap::PowerCap;

pub fn sub_run(args: &ArgMatches) -> ! {
    println!("measure mode");
    let delay_time = args
        .value_of("delay-time")
        .unwrap_or_default()
        .parse()
        .unwrap_or(2.0);

    let hwmon = coretemp::init();
    let mut powercap = PowerCap::new();
    sleep(Duration::from_millis(5)); // prevent powercap from div 0 at the first calculate, may lead to inaccurate result in the first show

    print!("\x1b[H\x1b[J");
    loop {
        powercap::print(&mut powercap);
        println!("\x1b[K");
        coretemp::print(&hwmon);
        // println!("{}", delay_time);
        print!("\x1b[J\x1b[H");
        sleep(Duration::from_secs_f64(delay_time));
    }
}
