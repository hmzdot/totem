use std::net::{SocketAddr, UdpSocket};

use message::{
    attribute::Value,
    header::{Header, HeaderType},
    Message,
};

pub struct Client {
    socket: UdpSocket,
    addrs: [SocketAddr; 4],
    credential: Option<Credential>,
}

impl Client {
    pub fn new(addrs: [SocketAddr; 4]) -> Self {
        let socket = UdpSocket::bind("0.0.0.0").expect("bind");
        Self {
            socket,
            addrs,
            credential: None,
        }
    }

    pub fn run(&mut self) {
        use HeaderType::*;

        let header = Header::with_random_id(BindingRequest);
        let message = Message::new(header, vec![]);
        self.socket
            .send_to(&message.encode(), self.addrs[0])
            .expect("send to");

        let mut buf = [0; 1024];
        let (amt, _) = self.socket.recv_from(&mut buf).unwrap();

        let buf = &buf[..amt];
        let message = Message::decode(buf);

        match &message.attributes[0].value {
            Value::MappedAddress(mapped) => {
                println!("My IP address is {:?}", mapped.address);
            }
            _ => panic!("invalid attribute"),
        }
    }
}

pub struct Credential(String, String);
