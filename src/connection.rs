use crate::error::TapsError;
use crate::preconnection::Preconnection;
use crate::preconnection::TransportInstance;
use crate::message::Message;

use async_std::{
    prelude::*,
};

pub struct Connection<'a, T, U> {
    preconnection: Preconnection<'a, T, U>,
    transport_instance: TransportInstance,
}

impl<'a, T, U> Connection<'a, T, U> {
    pub fn new(
        preconnection: Preconnection<'a, T, U>,
        transport_instance: TransportInstance,
    ) -> Connection::<'a, T, U> {
        Connection {
            preconnection: preconnection,
            transport_instance: transport_instance,
        }
    }

    pub async fn send(&mut self, message: Message<T>) -> Result<(), TapsError> {
        let send_data: Vec<u8> = self.preconnection.framer.new_sent_message(message);

        if self.transport_instance.tcp_stream_instance.is_some() {
            return self.send_message_tcp(send_data).await;
        } else if self.transport_instance.udp_socket_instance.is_some() {
            return self.send_message_udp(send_data).await;
        } else if self.transport_instance.quic_stream_instance.is_some() {
            return self.send_message_quic(send_data).await;
        }

        return Ok(())
    }

    pub async fn receive(&mut self) -> Result<Message<U>, TapsError> {
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

        return Ok(Message::<U>::new(self.preconnection.framer.handle_received_data(message_data), None));
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

    async fn send_message_quic(&mut self, message_data: Vec<u8>) -> Result<(), TapsError> {
        let quic_instance = self.transport_instance.quic_stream_instance.as_mut().unwrap();
        let mut conn = quic_instance.as_mut();

        // Handshake not completed
        if !conn.is_established() {
            return Err(TapsError::MessageSendFailed);
        }

        match conn.stream_send(0, &message_data, true) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(TapsError::MessageSendFailed),
        }        
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

    async fn receive_message_quic(&mut self) -> Result<Vec<u8>, TapsError> {
        let mut buf = vec![0u8; 1024];
        let conn = self.transport_instance.quic_stream_instance.as_mut().unwrap();
        let mut read = false;

        for stream_id in conn.readable() {
            if conn.stream_recv(stream_id, &mut buf).is_ok() {
                read = true;
                break;
            }
        }
        if read == false {
            return Err(TapsError::MessageReceiveFailed);
        }
        return Ok(buf);
    }
}
