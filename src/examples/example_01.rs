#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn run() {
    println!("\n\x1b[36;7m Running example 01... \x1b[0m");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four: {:?}, six: {:?}", four, six);

    route(four); // output -> Routing to V4
    route(IpAddrKind::V6); // output -> Routing to V6

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home: {:#?}", home);
    println!("loopback: {:#?}", loopback);
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing to {:?}", ip_kind);
}
