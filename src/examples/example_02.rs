#[derive(Debug)]
enum IpAddr {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn run() {
    println!("\n\x1b[36;7m Running example 02... \x1b[0m");

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:#?}", home);
    println!("loopback: {:#?}", loopback);
}
