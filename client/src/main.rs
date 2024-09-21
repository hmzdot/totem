mod client;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

use message::{
    attribute::Value,
    header::{Header, HeaderType},
    Message,
};

fn main() {
    // Connect to 192.168.1.148:3478 and send a simple UDP packet
    let a1 = IpAddr::V4(Ipv4Addr::new(172, 19, 0, 4));
    let p2 = 3479; // Alternate port for STUN
    let sockaddr = SocketAddr::new(a1, p2);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Socket bind failed");

    let header = Header::with_random_id(HeaderType::BindingRequest);
    let message = Message::new(header, vec![]);
    let _ = socket
        .send_to(&message.encode(), sockaddr)
        .expect("Send failed");
    println!("Sent packet to {:?}", sockaddr);

    let mut buf = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buf).unwrap();
    println!("Received {amt} bytes from {:?}", src);

    let buf = &buf[..amt];
    let message = Message::decode(buf);

    match &message.attributes[0].value {
        Value::MappedAddress(mapped) => {
            println!("My IP address is {:?}", mapped.address);
        }
        _ => panic!("Invalid value"),
    }

    println!("Response: {message:?}");
}
