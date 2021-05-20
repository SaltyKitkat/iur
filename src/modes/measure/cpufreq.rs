pub(crate) fn test(freq: &mut CpuFreqs) {
    freq.read();
    freq.fds
        .iter()
        .enumerate()
        .for_each(|(i, freq)| println!("Core {:<4}{:>8.3}MHz", i, freq.0))
}

use num_cpus;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};
pub struct CpuFreqs {
    fds: Vec<(u32, Option<File>)>,
}
impl CpuFreqs {
    pub fn init() -> CpuFreqs {
        let cpu_num = num_cpus::get();
        let paths = if Path::new("/sys/devices/system/cpu/cpufreq/policy0").exists() {
            let mut tmp = vec![];
            (0..cpu_num).for_each(|id| {
                tmp.push(PathBuf::from(format!(
                    "/sys/devices/system/cpu/cpufreq/policy{}/scaling_cur_freq",
                    id
                )))
            });
            tmp
        } else if Path::new("/sys/devices/system/cpu/cpu0/cpufreq").exists() {
            let mut tmp = vec![];
            (0..cpu_num).for_each(|id| {
                tmp.push(PathBuf::from(format!(
                    "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq",
                    id
                )))
            });
            tmp
        } else {
            todo!("support other ways")
        };
        let fds = paths
            .into_iter()
            .map(|path| (0, File::open(path).ok()))
            .collect();
        CpuFreqs { fds }
    }

    fn read_to_vec(&mut self, ret: &mut Vec<u32>) {
        self.fds.iter_mut().for_each(|fd| {
            if let (_, Some(fd)) = fd {
                let mut buf = String::with_capacity(16);
                fd.read_to_string(&mut buf).unwrap();
                let buf = &buf[0..buf.len() - 1];
                let raw_freq: u32 = buf.parse().unwrap();
                ret.push(raw_freq / 1000);
            }
        });
    }

    pub fn read(&mut self) {
        self.fds.iter_mut().for_each(|fd| {
            if let (freq, Some(fd)) = fd {
                let mut buf = String::with_capacity(16);
                fd.seek(SeekFrom::Start(0)).unwrap();
                fd.read_to_string(&mut buf).unwrap();
                let buf = &buf[0..buf.len() - 1];
                let raw_freq: u32 = buf.parse().unwrap();
                *freq = raw_freq / 1000;
            }
        });
    }
}
