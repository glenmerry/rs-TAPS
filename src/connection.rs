use crate::error::TapsError;
use crate::preconnection::Preconnection;
use crate::preconnection::TransportInstance;
use crate::message::Message;

use async_std::{
    prelude::*,
};

// #[derive(Debug)]
pub struct Connection<'a> {
    preconnection: Preconnection<'a>,
    transport_instance: TransportInstance,
}

impl<'a> Connection<'a> {
    pub fn new(
        preconnection: Preconnection<'a>,
        transport_instance: TransportInstance,
    ) -> Connection<'a> {
        Connection {
            preconnection: preconnection,
            transport_instance: transport_instance,
        }
    }

    pub async fn send(&self, message: Message) -> Result<(), TapsError> {
        let message_data: Vec<u8>;

        if self.preconnection.framer.is_some() {
            message_data = self.preconnection.framer.unwrap().new_sent_message(&self, &message);
        } else {
            message_data = message.data;
        }

        if self.transport_instance.tcp_stream_instance.is_some() {
            return self.send_message_tcp(message_data).await;
        } else if self.transport_instance.udp_socket_instance.is_some() {
            return self.send_message_udp(message_data).await;
        } else if self.transport_instance.quic_stream_instance.is_some() {
            return self.send_message_quic(message_data).await;
        }

        return Ok(())
    }

    pub async fn receive(&self) -> Result<Message, TapsError> {
        let mut result = Err(TapsError::MessageReceiveFailed);

        if self.transport_instance.tcp_stream_instance.is_some() {
            result = self.receive_message_tcp().await;
        } else if self.transport_instance.udp_socket_instance.is_some() {
            result = self.receive_message_udp().await;
        } else if self.transport_instance.quic_stream_instance.is_some() {
            result = self.receive_message_quic().await;
        }

        let message_data = match result {
            Ok(message_data) => message_data,
            Err(e) => return Err(e),
        };

        // deframe...

        return Ok(Message::new(message_data, None));
    }

    pub async fn close(&self) -> Result<(), TapsError> {
        todo!();
    }

    pub fn abort(self) {
        drop(self);
    }

    async fn send_message_tcp(&self, message_data: Vec<u8>) -> Result<(), TapsError> {
        match self.transport_instance.tcp_stream_instance.as_ref().unwrap().write(&message_data).await {
            Ok(_) => return Ok(()),
            Err(_) => return Err(TapsError::MessageSendFailed),
        }
    }

    async fn send_message_udp(&self, message_data: Vec<u8>) -> Result<(), TapsError> {
        match self.transport_instance.udp_socket_instance.as_ref().unwrap().send(&message_data).await {
            Ok(_) => return Ok(()),
            Err(_) => return Err(TapsError::MessageSendFailed),
        }
    }

    async fn send_message_quic(&self, message_data: Vec<u8>) -> Result<(), TapsError> {
        todo!();
        // let mut quic_instance = self.transport_instance.quic_stream_instance.unwrap();
        // let mut conn = quic_instance.as_mut();

        // // Handshake not completed
        // if !conn.is_established() {
        //     return Err(TapsError::MessageSendFailed);
        // }

        // match conn.stream_send(0, &message_data, true) {
        //     Ok(_) => return Ok(()),
        //     Err(_) => return Err(TapsError::MessageSendFailed),
        // }        
    }

    async fn receive_message_tcp(&self) -> Result<Vec<u8>, TapsError> {
        let mut buf = vec![0u8; 1024];
        match self.transport_instance.tcp_stream_instance.as_ref().unwrap().read(&mut buf).await {
            Ok(_) => return Ok(buf),
            Err(_) => return Err(TapsError::MessageReceiveFailed),
        }
    }

    async fn receive_message_udp(&self) -> Result<Vec<u8>, TapsError> {
        let mut buf = vec![0u8; 1024];
        match self.transport_instance.udp_socket_instance.as_ref().unwrap().recv(&mut buf).await {
            Ok(_) => return Ok(buf),
            Err(_) => return Err(TapsError::MessageReceiveFailed),
        }
    }

    async fn receive_message_quic(&self) -> Result<Vec<u8>, TapsError> {
        todo!();
        // let mut buf = vec![0u8; 1024];
        // let mut conn = self.transport_instance.quic_stream_instance.unwrap();
        // let mut read = false;

        // for stream_id in conn.readable() {
        //     if conn.stream_recv(stream_id, &mut buf).is_ok() {
        //         read = true;
        //         break;
        //     }
        // }
        // if read == false {
        //     return Err(TapsError::MessageReceiveFailed);
        // }
        // return Ok(buf);
    }
}
