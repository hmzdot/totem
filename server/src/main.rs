mod server;

use server::Server;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    let a1 = IpAddr::V4(Ipv4Addr::new(172, 19, 0, 2));
    let a2 = IpAddr::V4(Ipv4Addr::new(172, 19, 0, 4));
    let p1 = 3478; // Standard port for STUN
    let p2 = 3479; // Alternate port for STUN

    let mut server = Server::new([
        SocketAddr::new(a1, p1),
        SocketAddr::new(a1, p2),
        SocketAddr::new(a2, p1),
        SocketAddr::new(a2, p2),
    ]);
    server.run();
}
