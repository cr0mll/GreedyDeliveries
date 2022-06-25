
#[repr(u8)]
pub enum MessageType {
    PostBlockchain,
    RequestBlockchain
}

pub struct MessageHeader {
    message_type: MessageType,
    message_size: u32,
}

pub struct Message {
    header: MessageHeader,
    body: Vec<u8>
}