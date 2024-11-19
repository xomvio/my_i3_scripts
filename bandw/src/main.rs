use sysinfo::Networks;
fn main() {
    let mut nets = Networks::new_with_refreshed_list();

    loop {
        nets.refresh();
        let net = nets.get("enp9s0").unwrap();

        let down= minimalize(net.received());
        let up = minimalize(net.transmitted());
        let middle = " ".repeat(16 - down.len() - up.len());

        println!("⤓ {}{middle}{} ⤒", down, up);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn minimalize(by: u64) -> String {
    if by < 1024 {
        format!("{} B/s", by)
    } else if by < 1024 * 1024 {
        format!("{} KB/s", by / 1024)
    } else {
        format!("{} MB/s", by / (1024 * 1024))
    }
}