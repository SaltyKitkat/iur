use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub(crate) struct CpuFreq {
    freq: Vec<u32>,
    paths: Vec<PathBuf>,
}

impl CpuFreq {
    pub fn new() -> Result<CpuFreq, Box<dyn Error>> {
        fn sel_path() -> String {
            if Path::new("/sys/devices/system/cpu/cpufreq/policy0").exists() {
                String::from("/sys/devices/system/cpu/cpufreq/policy{}/scaling_cur_freq")
            } else if Path::new("/sys/devices/system/cpu/cpu0/cpufreq").exists() {
                String::from("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq")
            } else {
                todo!("not supported yet!") // maybe we can throw an error here instead
            }
        }

        let pathstr = sel_path();
        let num_cpus = num_cpus::get();
        let mut freq = Vec::with_capacity(num_cpus);
        let mut paths = Vec::with_capacity(num_cpus);
        for i in 0..num_cpus {
            let p = PathBuf::from(pathstr.clone().replace("{}", &i.to_string()));
            // unnecessary to read the value here.
            // but read it once on create to make sure
            // we both have permission and can parse it correctly
            // so it won't get an error when calling get().( in normal situation
            freq.push(
                fs::read_to_string(&p)?
                    .split('\n')
                    .next()
                    .unwrap()// read the first line of file
                    .parse()?,
            );
            paths.push(p);
        }
        Ok(CpuFreq { freq, paths })
    }

    // if get() returns a Result<>, we should deal with it in print()
    pub fn print(&mut self) {
        let freq_data = self.get();
        for (i, f) in freq_data.iter().enumerate() {
            println!("Cpu{:<8}{:>6}MHz\x1b[K", i, f / 1000);
        }
    }

    // maybe we should return Result<> instead?
    pub fn get(&mut self) -> &[u32] {
        // this is how we can mut borrow multi fields of a struct at the same time
        let CpuFreq { freq, paths } = self;
        for (f, p) in freq.into_iter().zip(paths) {
            *f = fs::read_to_string(p)
                .unwrap()
                .split('\n')
                .next()
                .unwrap()
                .parse()
                .unwrap();
        }
        &*freq
    }
}

