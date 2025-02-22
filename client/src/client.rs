use std::net::{SocketAddr, UdpSocket};

use message::{
    attribute::Value,
    header::{Header, HeaderType},
    Message,
};

pub struct Client {
    addrs: [SocketAddr; 4],
    credential: Option<Credential>,
}

impl Client {
    pub fn new(addrs: [SocketAddr; 4]) -> Self {
        Self {
            addrs,
            credential: None,
        }
    }

    pub fn run(&mut self) {
        use HeaderType::*;

        let socket = UdpSocket::bind("0.0.0.0:0").expect("bind");
        let header = Header::with_random_id(BindingRequest);
        let message = Message::new(header, vec![]);
        socket
            .send_to(&message.encode(), self.addrs[0])
            .expect("send to");

        let mut buf = [0; 1024];
        let (amt, _) = socket.recv_from(&mut buf).unwrap();

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
