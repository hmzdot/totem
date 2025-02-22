mod client;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use client::Client;

fn main() {
    let a1 = IpAddr::V4(Ipv4Addr::new(172, 19, 0, 2));
    let a2 = IpAddr::V4(Ipv4Addr::new(172, 19, 0, 4));
    let p1 = 3478; // Standard port for STUN
    let p2 = 3479; // Alternate port for STUN

    let mut client = Client::new([
        SocketAddr::new(a1, p1),
        SocketAddr::new(a1, p2),
        SocketAddr::new(a2, p1),
        SocketAddr::new(a2, p2),
    ]);
    client.run();
}
