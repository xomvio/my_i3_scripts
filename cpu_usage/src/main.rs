use std::{thread::sleep, time::Duration};

use sysinfo::{self, CpuExt, SystemExt};
fn main() {
    let mut sys = sysinfo::System::new();
    loop {
        sys.refresh_cpu();
        let mut total_use:f32 = 0.0;
        for cpu in sys.cpus() { total_use += cpu.cpu_usage(); }
        let usage:f32 = (total_use / sys.cpus().len() as f32).round();
        let space = if usage < 10.0 { " " } else { "" };
        println!("CPU {space}{usage}%");
        sleep(Duration::from_secs(1));
    }
}
