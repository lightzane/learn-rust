struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

pub fn run() {
    println!("\n\x1b[37;2;7m Running example 03... \x1b[0m");
}
