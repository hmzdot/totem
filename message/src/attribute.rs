use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Attribute {
    pub attr_type: AttrType,
    pub value: Value,
}

impl Attribute {
    pub const fn new(attr_type: AttrType, value: Value) -> Self {
        Self { attr_type, value }
    }

    /// Decode an attribute from a byte slice
    /// Returns the attribute and the number of bytes consumed
    pub fn decode(data: &[u8]) -> (Self, usize) {
        let attr_type = AttrType::from_be_bytes([data[0], data[1]]);
        let length = u16::from_be_bytes([data[2], data[3]]);
        let data = &data[4..(4 + length as usize)];
        let value = Value::decode(attr_type, data);
        (Attribute { attr_type, value }, 4 + length as usize)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice((self.attr_type as u16).to_be_bytes().as_ref());

        let value_bytes = self.value.encode();
        buf.extend_from_slice(&(value_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(&value_bytes);
        buf
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum AttrType {
    MappedAddress = 0x0001,
    ResponseAddress = 0x0002,
    ChangeRequest = 0x0003,
    SourceAddress = 0x0004,
    ChangedAddress = 0x0005,
    Username = 0x0006,
    Password = 0x0007,
    MessageIntegrity = 0x0008,
    ErrorCode = 0x0009,
    UnknownAttributes = 0x000A,
    ReflectedFrom = 0x000B,
}

impl AttrType {
    pub const fn from_be_bytes(bytes: [u8; 2]) -> AttrType {
        AttrType::from_u16(u16::from_be_bytes(bytes))
    }

    pub const fn from_u16(value: u16) -> AttrType {
        match value {
            0x0001 => AttrType::MappedAddress,
            0x0002 => AttrType::ResponseAddress,
            0x0003 => AttrType::ChangeRequest,
            0x0004 => AttrType::SourceAddress,
            0x0005 => AttrType::ChangedAddress,
            0x0006 => AttrType::Username,
            0x0007 => AttrType::Password,
            0x0008 => AttrType::MessageIntegrity,
            0x0009 => AttrType::ErrorCode,
            0x000A => AttrType::UnknownAttributes,
            0x000B => AttrType::ReflectedFrom,
            _ => panic!("Unknown attribute type"),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    MappedAddress(MappedAddress),
    ResponseAddress(ResponseAddress),
    ChangeRequest(ChangeRequest),
    SourceAddress(SourceAddress),
    ChangedAddress(ChangedAddress),
    Username(Username),
    Password(Password),
    MessageIntegrity(MessageIntegrity),
    ErrorCode(ErrorCode),
    UnknownAttributes(UnknownAttributes),
    ReflectedFrom(ReflectedFrom),
}

impl Value {
    pub fn decode(attr_type: AttrType, data: &[u8]) -> Value {
        match attr_type {
            AttrType::MappedAddress => Value::MappedAddress(MappedAddress::decode(data)),
            AttrType::ResponseAddress => Value::ResponseAddress(ResponseAddress::decode(data)),
            AttrType::ChangeRequest => Value::ChangeRequest(ChangeRequest::decode(data)),
            AttrType::SourceAddress => Value::SourceAddress(SourceAddress::decode(data)),
            AttrType::ChangedAddress => Value::ChangedAddress(ChangedAddress::decode(data)),
            AttrType::Username => Value::Username(Username::decode(data)),
            AttrType::Password => Value::Password(Password::decode(data)),
            AttrType::MessageIntegrity => Value::MessageIntegrity(MessageIntegrity::decode(data)),
            AttrType::ErrorCode => Value::ErrorCode(ErrorCode::decode(data)),
            AttrType::UnknownAttributes => {
                Value::UnknownAttributes(UnknownAttributes::decode(data))
            }
            AttrType::ReflectedFrom => Value::ReflectedFrom(ReflectedFrom::decode(data)),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        match self {
            Value::MappedAddress(value) => value.encode(),
            Value::ResponseAddress(value) => value.encode(),
            Value::ChangeRequest(value) => value.encode(),
            Value::SourceAddress(value) => value.encode(),
            Value::ChangedAddress(value) => value.encode(),
            Value::Username(value) => value.encode(),
            Value::Password(value) => value.encode(),
            Value::MessageIntegrity(value) => value.encode(),
            Value::ErrorCode(value) => value.encode(),
            Value::UnknownAttributes(value) => value.encode(),
            Value::ReflectedFrom(value) => value.encode(),
        }
    }

    pub fn into_attribute(self) -> Attribute {
        match self {
            Value::MappedAddress(_) => Attribute::new(AttrType::MappedAddress, self),
            Value::ResponseAddress(_) => Attribute::new(AttrType::ResponseAddress, self),
            Value::ChangeRequest(_) => Attribute::new(AttrType::ChangeRequest, self),
            Value::SourceAddress(_) => Attribute::new(AttrType::SourceAddress, self),
            Value::ChangedAddress(_) => Attribute::new(AttrType::ChangedAddress, self),
            Value::Username(_) => Attribute::new(AttrType::Username, self),
            Value::Password(_) => Attribute::new(AttrType::Password, self),
            Value::MessageIntegrity(_) => Attribute::new(AttrType::MessageIntegrity, self),
            Value::ErrorCode(_) => Attribute::new(AttrType::ErrorCode, self),
            Value::UnknownAttributes(_) => Attribute::new(AttrType::UnknownAttributes, self),
            Value::ReflectedFrom(_) => Attribute::new(AttrType::ReflectedFrom, self),
        }
    }
}

#[derive(Debug)]
pub struct MappedAddress {
    pub family: u8,
    pub port: u16,
    pub address: Ipv4Addr,
}

impl MappedAddress {
    pub const fn new(family: u8, port: u16, address: Ipv4Addr) -> Self {
        MappedAddress {
            family,
            port,
            address,
        }
    }

    pub fn decode(data: &[u8]) -> MappedAddress {
        let family = data[1];
        let port = u16::from_be_bytes([data[2], data[3]]);
        let address = Ipv4Addr::new(data[4], data[5], data[6], data[7]);
        MappedAddress::new(family, port, address)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(0);
        buf.push(self.family);
        buf.extend_from_slice(&self.port.to_be_bytes());
        buf.extend_from_slice(&self.address.octets());
        buf
    }
}

#[derive(Debug)]
pub struct ResponseAddress {
    pub family: u8,
    pub port: u16,
    pub address: Ipv4Addr,
}

impl ResponseAddress {
    pub const fn new(family: u8, port: u16, address: Ipv4Addr) -> Self {
        ResponseAddress {
            family,
            port,
            address,
        }
    }

    pub fn decode(data: &[u8]) -> ResponseAddress {
        let family = data[1];
        let port = u16::from_be_bytes([data[2], data[3]]);
        let address = Ipv4Addr::new(data[4], data[5], data[6], data[7]);
        ResponseAddress::new(family, port, address)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(0);
        buf.push(self.family);
        buf.extend_from_slice(&self.port.to_be_bytes());
        buf.extend_from_slice(&self.address.octets());
        buf
    }
}

#[derive(Debug)]
pub struct ChangedAddress {
    pub family: u8,
    pub port: u16,
    pub address: Ipv4Addr,
}

impl ChangedAddress {
    pub const fn new(family: u8, port: u16, address: Ipv4Addr) -> Self {
        ChangedAddress {
            family,
            port,
            address,
        }
    }

    pub fn decode(data: &[u8]) -> ChangedAddress {
        let family = data[1];
        let port = u16::from_be_bytes([data[2], data[3]]);
        let address = Ipv4Addr::new(data[4], data[5], data[6], data[7]);
        ChangedAddress::new(family, port, address)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(0);
        buf.push(self.family);
        buf.extend_from_slice(&self.port.to_be_bytes());
        buf.extend_from_slice(&self.address.octets());
        buf
    }
}

#[derive(Debug)]
pub struct ChangeRequest {
    pub change_ip: bool,
    pub change_port: bool,
}

impl ChangeRequest {
    pub const fn new(change_ip: bool, change_port: bool) -> Self {
        ChangeRequest {
            change_ip,
            change_port,
        }
    }

    pub fn decode(data: &[u8]) -> ChangeRequest {
        let change_ip = data[3] & 0x04 != 0;
        let change_port = data[3] & 0x02 != 0;
        ChangeRequest {
            change_ip,
            change_port,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0, 0, 0, 0]);
        if self.change_ip {
            buf[3] |= 0x04;
        }
        if self.change_port {
            buf[3] |= 0x02;
        }
        buf
    }
}

#[derive(Debug)]
pub struct SourceAddress {
    pub family: u8,
    pub port: u16,
    pub address: Ipv4Addr,
}

impl SourceAddress {
    pub const fn new(family: u8, port: u16, address: Ipv4Addr) -> Self {
        SourceAddress {
            family,
            port,
            address,
        }
    }

    pub fn decode(data: &[u8]) -> SourceAddress {
        let family = data[1];
        let port = u16::from_be_bytes([data[2], data[3]]);
        let address = Ipv4Addr::new(data[4], data[5], data[6], data[7]);
        SourceAddress::new(family, port, address)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(0);
        buf.push(self.family);
        buf.extend_from_slice(&self.port.to_be_bytes());
        buf.extend_from_slice(&self.address.octets());
        buf
    }
}

#[derive(Debug)]
pub struct Username {
    pub username: String,
}

impl Username {
    pub const fn new(username: String) -> Self {
        Username { username }
    }

    pub fn decode(data: &[u8]) -> Username {
        let username = String::from_utf8(data.to_vec()).unwrap();
        Username::new(username)
    }

    pub fn encode(&self) -> Vec<u8> {
        self.username.as_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct Password {
    pub password: String,
}

impl Password {
    pub const fn new(password: String) -> Self {
        Password { password }
    }

    pub fn decode(data: &[u8]) -> Password {
        let password = String::from_utf8(data.to_vec()).unwrap();
        Password::new(password)
    }

    pub fn encode(&self) -> Vec<u8> {
        self.password.as_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct MessageIntegrity {
    pub integrity: [u8; 20],
}

impl MessageIntegrity {
    pub const fn new(integrity: [u8; 20]) -> Self {
        MessageIntegrity { integrity }
    }

    pub fn decode(data: &[u8]) -> MessageIntegrity {
        let mut integrity = [0; 20];
        integrity.copy_from_slice(&data[0..20]);
        MessageIntegrity::new(integrity)
    }

    pub fn encode(&self) -> Vec<u8> {
        self.integrity.to_vec()
    }
}

#[derive(Debug)]
pub struct ErrorCode {
    pub code: u16,
    pub reason: String,
}

impl ErrorCode {
    pub const fn new(code: u16, reason: String) -> Self {
        ErrorCode { code, reason }
    }

    pub fn decode(data: &[u8]) -> ErrorCode {
        let class = data[2];
        let number = data[3];
        let code = u16::from(class) * 100 + u16::from(number);
        let reason = String::from_utf8(data[4..].to_vec()).unwrap();
        ErrorCode::new(code, reason)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0, 0, self.class(), self.number()]);
        buf.extend_from_slice(self.reason.as_bytes());
        buf
    }

    pub const fn class(&self) -> u8 {
        (self.code / 100) as u8
    }

    pub const fn number(&self) -> u8 {
        (self.code % 100) as u8
    }
}

#[derive(Debug)]
pub struct UnknownAttributes {
    pub attributes: Vec<u16>,
}

impl UnknownAttributes {
    pub const fn new(attributes: Vec<u16>) -> Self {
        UnknownAttributes { attributes }
    }

    pub fn decode(data: &[u8]) -> UnknownAttributes {
        let mut attributes = Vec::new();
        for i in 0..data.len() / 2 {
            attributes.push(u16::from_be_bytes([data[i * 2], data[i * 2 + 1]]));
        }
        UnknownAttributes::new(attributes)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        for attr in &self.attributes {
            buf.extend_from_slice(&attr.to_be_bytes());
        }
        buf
    }
}

#[derive(Debug)]
pub struct ReflectedFrom {
    pub family: u8,
    pub port: u16,
    pub address: Ipv4Addr,
}

impl ReflectedFrom {
    pub const fn new(family: u8, port: u16, address: Ipv4Addr) -> Self {
        ReflectedFrom {
            family,
            port,
            address,
        }
    }

    pub fn decode(data: &[u8]) -> ReflectedFrom {
        let family = data[1];
        let port = u16::from_be_bytes([data[2], data[3]]);
        let address = Ipv4Addr::new(data[4], data[5], data[6], data[7]);
        ReflectedFrom::new(family, port, address)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(0);
        buf.push(self.family);
        buf.extend_from_slice(&self.port.to_be_bytes());
        buf.extend_from_slice(&self.address.octets());
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_mapped_address_encode_decode() {
        let mapped_address = MappedAddress::new(1, 8080, Ipv4Addr::new(192, 168, 0, 1));
        let encoded = Value::MappedAddress(mapped_address).encode();
        let decoded = Value::decode(AttrType::MappedAddress, &encoded);

        if let Value::MappedAddress(decoded_address) = decoded {
            assert_eq!(decoded_address.family, 1);
            assert_eq!(decoded_address.port, 8080);
            assert_eq!(decoded_address.address, Ipv4Addr::new(192, 168, 0, 1));
        } else {
            panic!("Decoded value is not a MappedAddress");
        }
    }

    #[test]
    fn test_change_request_encode_decode() {
        let change_request = ChangeRequest::new(true, false);
        let encoded = Value::ChangeRequest(change_request).encode();
        let decoded = Value::decode(AttrType::ChangeRequest, &encoded);

        if let Value::ChangeRequest(decoded_request) = decoded {
            assert_eq!(decoded_request.change_ip, true);
            assert_eq!(decoded_request.change_port, false);
        } else {
            panic!("Decoded value is not a ChangeRequest");
        }
    }

    #[test]
    fn test_username_encode_decode() {
        let username = Username::new("testuser".to_string());
        let encoded = Value::Username(username).encode();
        let decoded = Value::decode(AttrType::Username, &encoded);

        if let Value::Username(decoded_username) = decoded {
            assert_eq!(decoded_username.username, "testuser");
        } else {
            panic!("Decoded value is not a Username");
        }
    }

    #[test]
    fn test_password_encode_decode() {
        let password = Password::new("testpassword".to_string());
        let encoded = Value::Password(password).encode();
        let decoded = Value::decode(AttrType::Password, &encoded);

        if let Value::Password(decoded_password) = decoded {
            assert_eq!(decoded_password.password, "testpassword");
        } else {
            panic!("Decoded value is not a Password");
        }
    }

    #[test]
    fn test_message_integrity_encode_decode() {
        let integrity = [1u8; 20];
        let message_integrity = MessageIntegrity::new(integrity);
        let encoded = Value::MessageIntegrity(message_integrity).encode();
        let decoded = Value::decode(AttrType::MessageIntegrity, &encoded);

        if let Value::MessageIntegrity(decoded_integrity) = decoded {
            assert_eq!(decoded_integrity.integrity, integrity);
        } else {
            panic!("Decoded value is not a MessageIntegrity");
        }
    }

    #[test]
    fn test_error_code_encode_decode() {
        let error_code = ErrorCode::new(400, "Bad Request".to_string());
        let encoded = Value::ErrorCode(error_code).encode();
        let decoded = Value::decode(AttrType::ErrorCode, &encoded);

        if let Value::ErrorCode(decoded_error) = decoded {
            assert_eq!(decoded_error.code, 400);
            assert_eq!(decoded_error.reason, "Bad Request");
        } else {
            panic!("Decoded value is not an ErrorCode");
        }
    }

    #[test]
    fn test_unknown_attributes_encode_decode() {
        let unknown_attrs = UnknownAttributes::new(vec![0x0001, 0x0002, 0x0003]);
        let encoded = Value::UnknownAttributes(unknown_attrs).encode();
        let decoded = Value::decode(AttrType::UnknownAttributes, &encoded);

        if let Value::UnknownAttributes(decoded_attrs) = decoded {
            assert_eq!(decoded_attrs.attributes, vec![0x0001, 0x0002, 0x0003]);
        } else {
            panic!("Decoded value is not UnknownAttributes");
        }
    }
}
