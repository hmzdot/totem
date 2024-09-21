use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

use message::{
    attribute::{ErrorCode, MappedAddress, Value},
    header::{Header, HeaderType},
    Message,
};

type UserMap = Arc<Mutex<HashMap<String, String>>>;

pub struct Server {
    sockets: [SocketAddr; 4],
    users: UserMap,
}

impl Server {
    pub fn new(sockets: [SocketAddr; 4]) -> Self {
        Self {
            sockets,
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn run(&mut self) {
        let mut threads = Vec::with_capacity(self.sockets.len());
        for socket in self.sockets {
            let users = self.users.clone();
            let sock = UdpSocket::bind(socket).expect("Socket failed to bind");
            let tid = std::thread::spawn(move || listen_udp(users, sock));
            threads.push(tid);
        }
        for thread in threads {
            thread.join().expect("thread join");
        }
    }
}

fn listen_udp(users: UserMap, sock: UdpSocket) {
    println!("Listening on {:?}", sock.local_addr().unwrap());

    let mut buf = [0; 1024];
    loop {
        let (amt, src) = sock.recv_from(&mut buf).expect("recv data");
        let request = Request::new(&sock, users.clone(), src);
        assert!(amt <= 1024, "request too big");

        let buf = &mut buf[..amt];
        let message = Message::decode(&buf);
        request.dispatch(message);
    }
}

struct Request<'a> {
    socket: &'a UdpSocket,
    _users: UserMap,
    src: SocketAddr,
}

impl<'a> Request<'a> {
    fn new(socket: &'a UdpSocket, users: UserMap, src: SocketAddr) -> Self {
        Self {
            socket,
            _users: users,
            src,
        }
    }

    fn dispatch(&self, message: Message) {
        match &message.header.header_type {
            HeaderType::BindingRequest => self.handle_binding(message),
            HeaderType::SharedSecretRequest => self.handle_shared(message),
            _ => panic!("invalid request"),
        }
    }

    fn handle_binding(&self, message: Message) {
        use HeaderType::*;
        let tx_id = message.header.transaction_id;

        let ip = match self.src.ip() {
            IpAddr::V4(v4) => v4,
            _ => {
                let header = Header::new(BindingErrorResponse, tx_id);
                let err = Value::ErrorCode(ErrorCode::new(400, "ipv4 only".into()));
                let message = Message::new(header, vec![err.into_attribute()]);
                self.socket
                    .send_to(&message.encode(), self.src)
                    .expect("send to");
                return;
            }
        };

        let header = Header::new(BindingResponse, tx_id);
        let mapped = Value::MappedAddress(MappedAddress::new(1, self.src.port(), ip));
        let message = Message::new(header, vec![mapped.into_attribute()]);
        self.socket
            .send_to(&message.encode(), self.src)
            .expect("send to");
    }

    fn handle_shared(&self, _message: Message) {
        todo!("handle shared")
    }
}
