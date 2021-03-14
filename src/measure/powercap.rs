use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

#[derive(Debug, Default)]
struct PowerSensor {
    name: String,
    path: PathBuf,
    last: u64,
}

impl PowerSensor {
    fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            last: 0,
        }
    }
}
#[derive(Debug)]
pub struct PowerCap(Vec<PowerSensor>, SystemTime);

impl PowerCap {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    fn new() -> PowerCap {
        PowerCap(vec![], SystemTime::now())
    }
}

pub fn powercap_init() -> PowerCap {
    let mut powercap = PowerCap::new();
    let tmp = Path::new("/sys/class/powercap");
    let mut powercap_dir = tmp.read_dir().expect("Failed to open powercap directory");
    if powercap_dir.all(|d| d.is_err()) {
        panic!("Failed to open powercap directory");
    }
    for entry in powercap_dir {
        match entry {
            Ok(dir)
                if {
                    let tmp = dir.file_name();
                    let tmp2 = tmp.to_string_lossy();
                    tmp2.contains(":") && !tmp2.contains("mmio")
                } =>
            {
                let mut name_path = dir.path();
                name_path.push("name");
                let name = if let Ok(mut name) = read_to_string(&name_path) {
                    name.pop(); //remove the endding '\n'
                    name
                } else {
                    eprintln!("failed to read from {}", name_path.to_string_lossy());
                    continue;
                };
                let mut path = dir.path();
                path.push("energy_uj");
                let path = path;
                powercap.0.push(PowerSensor::new(name, path));
            }
            _ => (),
        }
    }
    for i in &mut powercap.0 {
        let tmp = match read_to_string(&i.path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let tmp = &tmp[0..tmp.len() - 1];
        i.last = tmp.parse().unwrap_or_default();
    }

    powercap
}

pub fn powercap_print(powercap: &mut PowerCap) {
    if powercap.len() == 0 {
        return;
    }
    let curtime = SystemTime::now();
    let extime = powercap.1;
    let diftime = curtime.duration_since(extime).unwrap_or_default();
    // let diftime = match powercap.1 {
    //     Some(extime) => curtime
    //         .duration_since(extime)
    //         .unwrap_or(Duration::from_secs(0)),
    //     None => Duration::from_secs(0),
    // };
    for i in &mut powercap.0 {
        // let tmp = read_to_string(&i.path).unwrap_or_default();
        let tmp = match read_to_string(&i.path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let tmp = &tmp[0..tmp.len() - 1];
        // tmp.pop();
        let cur = tmp.parse().unwrap_or_default();
        if !(cur == 0 || diftime.as_secs() == 0) {
            println!(
                "{:<13}{:>6}mW\x1b[K",
                i.name,
                (cur - i.last) / (diftime.as_secs() * 1000)
            );
        }
        i.last = cur;
    }
    powercap.1 = curtime;
}
