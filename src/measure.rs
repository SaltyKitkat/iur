use std::{thread::sleep, time::Duration};

use clap::ArgMatches;
use libmedium::{
    hwmon::Hwmon,
    parse_hwmons,
    sensors::{Input, Sensor},
    units::Temperature,
};
pub fn sub_run(args: &ArgMatches) -> ! {
    println!("measure mode");
    let tmp = parse_hwmons().expect("Failed to read from hwmon!");
    let mut hwmon = tmp.hwmons_by_name("coretemp");
    let hwmon = hwmon.next().expect("Failed to read from hwmon!");
    print!("\x1b[H\x1b[J");
    loop {
        print_hwmon(&hwmon);
        print!("\x1b[J\x1b[H");
        sleep(Duration::from_secs(2));
    }
}
fn print_hwmon(hwmon: &Hwmon) {
    for (id, temp_sensor) in hwmon.temps() {
        println!(
            "{:<15}{}\x1b[K",
            temp_sensor.name(),
            temp_sensor
                .read_input()
                .unwrap_or(Temperature::from_degrees_celsius(0))
        );
    }
}
