use std::{
    fs::{read_to_string, File},
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Debug, Default)]
struct PowerSensor {
    name: String,
    fd: Option<File>,
    last: u64,
}

impl PowerSensor {
    fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            fd: File::open(path).ok(),
            last: 0,
        }
    }
}
#[derive(Debug)]
pub(crate) struct PowerCap(Vec<PowerSensor>, SystemTime);

impl PowerCap {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn new() -> PowerCap {
        let mut powercap = PowerCap(vec![], SystemTime::now());
        let tmp = Path::new("/sys/class/powercap");
        let powercap_dir = tmp.read_dir().expect("Failed to open powercap directory");
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
                    powercap.0.push(PowerSensor::new(name, path));
                }
                _ => (),
            }
        }
        for i in &mut powercap.0 {
            let tmp: String = match &mut i.fd {
                Some(fd) => {
                    let mut buf = String::new();
                    match fd.read_to_string(&mut buf) {
                        Ok(_) => buf,
                        Err(_) => continue,
                    }
                }
                None => continue,
            };
            let tmp = &tmp[0..tmp.len() - 1];
            i.last = tmp.parse().unwrap_or_default();
        }
        powercap
    }
}

pub(crate) fn print(powercap: &mut PowerCap) {
    if powercap.len() == 0 {
        return;
    }
    let curtime = SystemTime::now();
    let extime = powercap.1;
    let diftime = curtime.duration_since(extime).unwrap_or_default();
    for i in &mut powercap.0 {
        let tmp: String = match &mut i.fd {
            Some(fd) => {
                fd.seek(SeekFrom::Start(0)).unwrap();
                let mut buf = String::new();
                match fd.read_to_string(&mut buf) {
                    Ok(_) => buf,
                    Err(_) => continue,
                }
            }
            None => continue,
        };
        let tmp = &tmp[0..tmp.len() - 1]; // remove the ending '\n'
        let cur = tmp.parse().unwrap_or_default();
        if !(cur == 0 || diftime.as_millis() == 0) {
            println!(
                "{:<13}{:>6.3}W\x1b[K",
                i.name,
                ((cur - i.last) / diftime.as_millis() as u64) as f64 / 1000.0
            );
        }
        i.last = cur;
    }
    powercap.1 = curtime;
}
