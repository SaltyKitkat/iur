use cpu_freq;
use cpu_freq::CpuFreqs;

pub(crate) fn test() {
    let tmp = cpu_freq::get();
    tmp.iter()
        .enumerate()
        .for_each(|(i, CpuFreqs { cur, .. })| {
            println!(
                "Core {:<4}{:>8.3}MHz",
                i,
                match cur {
                    Some(v) => v,
                    None => unreachable!(),
                }
            );
        });
}
