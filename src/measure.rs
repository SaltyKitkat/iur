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
    let delay_time = args
        .value_of("delay-time")
        .unwrap_or_default()
        .parse()
        .unwrap_or(2.0);

    // let tmp = Path::new("/sys/class/powercap");
    // let mut powercap = tmp.read_dir().expect("Failed to open powercap directory");
    // if powercap.all(|d| d.is_err()) {
    //     panic!("Failed to open powercap directory");
    // }

    let hwmon = coretemp_init();

    print!("\x1b[H\x1b[J");
    loop {
        // powercap_print(&mut powercap);
        coretemp_print(&hwmon);
        println!("{}", delay_time);
        print!("\x1b[J\x1b[H");
        sleep(Duration::from_secs_f64(delay_time));
    }
}

// fn powercap_print(powercap: &mut ReadDir) {
//     for entry in powercap {
//         match entry {
//             Ok(dir) if dir.file_name().to_string_lossy().contains(":") => {
//                 let mut name_path = dir.path();
//                 name_path.push("name");
//                 let powercap_name = read_to_string(name_path);
//             }
//             _ => (),
//         }
//     }
// }

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
