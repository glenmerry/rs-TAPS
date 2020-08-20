use crate::message_context::MessageContext;

use http::{Request, Response};

// #[derive(Debug, Clone)]
// pub struct Message {
//     pub data: Vec<u8>,
//     message_context: Option<MessageContext>,
// }

// impl Message {
//     pub fn new(data: Vec<u8>, message_context: Option<MessageContext>) -> Message {
//         Message {
//             data: data,
//             message_context: message_context,
//         }
//     }
// }

pub trait Message {
    fn as_bytes(&self) -> Vec<u8>;
}

pub struct HttpRequestMessage {
    http_request: Request<Vec<u8>>,
    message_context: Option<MessageContext>,
}

impl Message for HttpRequestMessage {
    fn as_bytes(&self) -> Vec<u8> {
        let mut request_bytes = Vec::new();

        let (parts, body) = self.http_request.into_parts();

        request_bytes.extend_from_slice(parts.method.as_str().as_bytes());
        request_bytes.extend_from_slice(b" / HTTP/1.1\r\n");
    }
}
