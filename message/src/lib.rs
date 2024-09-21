pub mod attribute;
pub mod header;

use attribute::Attribute;
use header::{Header, HeaderType};

#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub attributes: Vec<Attribute>,
}

impl Message {
    pub const fn new(header: Header, attributes: Vec<Attribute>) -> Self {
        Self { header, attributes }
    }

    pub fn decode(data: &[u8]) -> Self {
        let header_type = HeaderType::from_be_bytes([data[0], data[1]]);
        let message_length = u16::from_be_bytes([data[2], data[3]]);
        let transaction_id = data[4..20].try_into().unwrap();
        let header = Header::new(header_type, transaction_id);

        let data = &data[20..];
        let mut attributes = Vec::new();
        let mut attr_read = 0;
        while attr_read < message_length as usize {
            let (attr, len) = Attribute::decode(&data[attr_read as usize..]);
            attributes.push(attr);
            attr_read += len;
        }

        Self::new(header, attributes)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut attr_data = Vec::new();
        for attr in &self.attributes {
            attr_data.extend_from_slice(&attr.encode());
        }
        let message_length = attr_data.len() as u16;

        let mut data = Vec::new();
        data.extend_from_slice(&(self.header.header_type as u16).to_be_bytes());
        data.extend_from_slice(&message_length.to_be_bytes());
        data.extend_from_slice(&self.header.transaction_id);
        data.extend_from_slice(&attr_data);
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attribute::{ChangeRequest, ErrorCode, MappedAddress, Username, Value};
    use std::net::Ipv4Addr;

    #[test]
    fn test_message_encode_decode_multiple_attributes() {
        // Create a message with multiple attributes
        let header = Header::new(HeaderType::BindingResponse, [1; 16]);
        let attributes = vec![
            Value::MappedAddress(MappedAddress::new(1, 8080, Ipv4Addr::new(192, 168, 0, 1)))
                .into_attribute(),
            Value::ChangeRequest(ChangeRequest::new(true, false)).into_attribute(),
            Value::Username(Username::new("testuser".to_string())).into_attribute(),
            Value::ErrorCode(ErrorCode::new(400, "Bad Request".to_string())).into_attribute(),
        ];
        let original_message = Message::new(header, attributes);

        // Encode the message
        let encoded = original_message.encode();

        // Decode the message
        let decoded_message = Message::decode(&encoded);

        // Verify the header
        assert_eq!(
            decoded_message.header.header_type,
            HeaderType::BindingResponse
        );
        assert_eq!(decoded_message.header.transaction_id, [1; 16]);

        // Verify the attributes
        assert_eq!(decoded_message.attributes.len(), 4);

        // Check MappedAddress
        if let Value::MappedAddress(addr) = &decoded_message.attributes[0].value {
            assert_eq!(addr.family, 1);
            assert_eq!(addr.port, 8080);
            assert_eq!(addr.address, Ipv4Addr::new(192, 168, 0, 1));
        } else {
            panic!("First attribute is not MappedAddress");
        }

        // Check ChangeRequest
        if let Value::ChangeRequest(req) = &decoded_message.attributes[1].value {
            assert_eq!(req.change_ip, true);
            assert_eq!(req.change_port, false);
        } else {
            panic!("Second attribute is not ChangeRequest");
        }

        // Check Username
        if let Value::Username(username) = &decoded_message.attributes[2].value {
            assert_eq!(username.username, "testuser");
        } else {
            panic!("Third attribute is not Username");
        }

        // Check ErrorCode
        if let Value::ErrorCode(error) = &decoded_message.attributes[3].value {
            assert_eq!(error.code, 400);
            assert_eq!(error.reason, "Bad Request");
        } else {
            panic!("Fourth attribute is not ErrorCode");
        }
    }
}
