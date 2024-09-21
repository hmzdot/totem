#[derive(Debug)]
pub struct Header {
    pub header_type: HeaderType,
    pub transaction_id: [u8; 16],
}

impl Header {
    pub const fn new(header_type: HeaderType, transaction_id: [u8; 16]) -> Self {
        Self {
            header_type,
            transaction_id,
        }
    }

    pub fn with_random_id(header_type: HeaderType) -> Self {
        let transaction_id: [u8; 16] = rand::random();
        Self {
            header_type,
            transaction_id,
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeaderType {
    BindingRequest = 0x0001,
    BindingResponse = 0x0101,
    BindingErrorResponse = 0x0111,
    SharedSecretRequest = 0x0002,
    SharedSecretResponse = 0x0102,
    SharedSecretErrorResponse = 0x0112,
}

impl HeaderType {
    pub fn from_be_bytes(bytes: [u8; 2]) -> Self {
        Self::from_u16(u16::from_be_bytes(bytes))
    }

    pub fn from_u16(value: u16) -> Self {
        match value {
            0x0001 => Self::BindingRequest,
            0x0101 => Self::BindingResponse,
            0x0111 => Self::BindingErrorResponse,
            0x0002 => Self::SharedSecretRequest,
            0x0102 => Self::SharedSecretResponse,
            0x0112 => Self::SharedSecretErrorResponse,
            _ => panic!("Invalid HeaderType value: {}", value),
        }
    }
}
