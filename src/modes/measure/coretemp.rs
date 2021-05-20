use libmedium::{
    hwmon::Hwmon,
    parse_hwmons,
    sensors::{Input, Sensor},
    units::Temperature,
};

pub(crate) fn init() -> Hwmon {
    let tmp = parse_hwmons().expect("Failed to read from hwmon!");
    let x = tmp
        .hwmons_by_name("coretemp")
        .next()
        .expect("Failed to read from hwmon!")
        .to_owned();
    x
}

pub fn print(hwmon: &Hwmon) {
    for (_id, temp_sensor) in hwmon.temps() {
        println!(
            "{:<12}{:>6}°C\x1b[K",
            temp_sensor.name(),
            temp_sensor
                .read_input()
                .unwrap_or(Temperature::from_degrees_celsius(0))
                .as_degrees_celsius()
        );
    }
}
