use std::{thread::sleep, time::Duration};

use clap::ArgMatches;
use libmedium::{
    hwmon::Hwmon,
    parse_hwmons,
    sensors::{Input, Sensor},
    units::Temperature,
};
use powercap::{powercap_init, powercap_print};
mod powercap;

pub fn sub_run(args: &ArgMatches) -> ! {
    println!("measure mode");
    let delay_time = args
        .value_of("delay-time")
        .unwrap_or_default()
        .parse()
        .unwrap_or(2.0);

    let hwmon = coretemp_init();
    let mut powercap = powercap_init();

    print!("\x1b[H\x1b[J");
    loop {
        powercap_print(&mut powercap);
        println!("\x1b[K");
        coretemp_print(&hwmon);
        println!("{}", delay_time);
        print!("\x1b[J\x1b[H");
        sleep(Duration::from_secs_f64(delay_time));
    }
}

fn coretemp_init() -> Hwmon {
    let tmp = parse_hwmons().expect("Failed to read from hwmon!");
    let x = tmp
        .hwmons_by_name("coretemp")
        .next()
        .expect("Failed to read from hwmon!")
        .to_owned();
    x
}

fn coretemp_print(hwmon: &Hwmon) {
    for (_id, temp_sensor) in hwmon.temps() {
        println!(
            "{:<13}{:>6}°C\x1b[K",
            temp_sensor.name(),
            temp_sensor
                .read_input()
                .unwrap_or(Temperature::from_degrees_celsius(0))
                .as_degrees_celsius()
        );
    }
}
