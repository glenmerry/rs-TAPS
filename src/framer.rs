use crate::message::Message;

use http::{Request, Response, StatusCode, Version};

pub trait Framer<T, U> {
    fn new_sent_message(&self, message: Message<T>) -> Vec<u8>;
    fn handle_received_data(&self, received_data: Vec<u8>) -> U;
}

pub struct HttpClientFramer;

impl<T: Encode, U: Decode> Framer<T, U> for HttpClientFramer {
    fn new_sent_message(&self, message: Message<T>) -> Vec<u8> {
        return message.data.encode();
    }

    fn handle_received_data(&self, received_data: Vec<u8>) -> U {
        return U::decode(received_data);
    }
}

pub trait Encode {
    fn encode(self) -> Vec<u8>;
}

impl Encode for Request<()> {
    fn encode(self) -> Vec<u8> {
        let mut request_bytes = Vec::new();
        let (parts, _body) = self.into_parts();

        request_bytes.extend_from_slice(parts.method.as_str().as_bytes());
        request_bytes.extend_from_slice(b" / HTTP/1.1\r\n");
        request_bytes.extend_from_slice(b"\r\nHost: ");
        request_bytes.extend_from_slice(format!("{}", parts.uri).as_bytes());
        request_bytes.extend_from_slice(b"\r\n\r\n");
        return request_bytes;
    }
}

pub trait Decode {
    fn decode(data: Vec<u8>) -> Self;
}

impl Decode for Response<()> {
    fn decode(data: Vec<u8>) -> Self {
        let response_str = std::str::from_utf8(&data).unwrap();
        let mut response_lines = response_str.lines();
        let mut status_line = response_lines.next().unwrap().split_whitespace();
        let version = match status_line.next().unwrap() {
            "HTTP/0.9" => Version::HTTP_09,
            "HTTP/1.0" => Version::HTTP_10,
            "HTTP/1.1" => Version::HTTP_11,
            "HTTP/2.0" => Version::HTTP_2,
            "HTTP/3.0" => Version::HTTP_3,
            _ => Version::HTTP_11,
        };
        let status_code = StatusCode::from_u16(status_line.next().unwrap().parse::<u16>().unwrap()).unwrap();

        let mut response = Response::builder()
            .version(version)
            .status(status_code);

        for header in response_lines {
            // End of headers
            if header.trim().is_empty() {
                break;
            }
            
            let mut split = header.split(": ");            
            response = response.header(split.next().unwrap(), split.next().unwrap());
        }
        
        return response
            .body(())
            .unwrap();
    }
}
