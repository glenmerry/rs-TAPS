use crate::preconnection::Preconnection;
use crate::connection::Connection;
use crate::error::TapsError;
use crate::preconnection::TransportInstance;

use std::pin::Pin;

use async_std::{
    stream::Stream,
    task,
    task::{Context, Poll},
    net::{SocketAddr, ToSocketAddrs, TcpListener}
};

use futures::stream::{FuturesUnordered, StreamExt};

pub struct Listener<'a, T, U> {
    preconnection: Preconnection<'a, T, U>,
    allowed_remote_addrs: Vec<SocketAddr>,
    tcp_listener: Option<TcpListener>,
}

impl<'a, T, U> Listener<'a, T, U> {
    pub fn new(
        preconnection: Preconnection<'a, T, U>,
    ) -> Listener<'a, T, U> {
        Listener::<T, U> {
            preconnection: preconnection,
            allowed_remote_addrs: vec![],
            tcp_listener: None,
        }
    }

    pub async fn start(&mut self) -> Result<(), TapsError> {
        let local_port = self.preconnection.local_endpoint.as_ref().unwrap().port.as_ref().unwrap();
        let local_addr = self.preconnection.local_endpoint.as_ref().unwrap().address.as_ref().unwrap();
        let local_socket_addr = format!("{}:{}", local_addr, local_port).to_socket_addrs().await?.next().unwrap();
        let candidate_protocol_ranks = self.preconnection.calculate_candidate_protocol_ranks()?;
        let candidate_protocols = candidate_protocol_ranks.keys();

        // Gather allowed remote endpoints - if provided
        if self.preconnection.remote_endpoint.is_some() {
            let remote_port = self.preconnection.remote_endpoint.as_ref().unwrap().port.as_ref().unwrap();

            // IP address provided in remote endpoint
            if self.preconnection.remote_endpoint.as_ref().unwrap().address.is_some() {
                let remote_addr = self.preconnection.remote_endpoint.as_ref().unwrap().address.as_ref().unwrap();
                let from_addr = format!("{}:{}", remote_addr, remote_port).to_socket_addrs().await?;
                for a in from_addr {
                    self.allowed_remote_addrs.push(a);
                }
            }

            // Host name provided in remote endpoint - DNS lookup performed here
            if self.preconnection.remote_endpoint.as_ref().unwrap().host_name.is_some() {
                let host_name = self.preconnection.remote_endpoint.as_ref().unwrap().host_name.as_ref().unwrap();
                let from_host_name = format!("{}:{}", host_name, remote_port).to_socket_addrs().await?;
                for a in from_host_name {
                    self.allowed_remote_addrs.push(a);
                }
            }
        }

        for protocol in candidate_protocols {
            match *protocol {
                "tcp" => {
                    self.tcp_listener = Some(TcpListener::bind(local_socket_addr).await?);
                },
                "udp" => {/* TODO */},
                "quic" => {/* TODO */},
                _ => {},
            }
        }

        return Ok(());
    }

    async fn poll_tcp(&self) -> Connection<'a, T, U> {
        loop {
            let incoming_conn = self.tcp_listener.as_ref().unwrap().accept().await.unwrap();
            if self.allowed_remote_addrs.contains(&incoming_conn.1) {
                return Connection::new(
                    Preconnection::new(self.preconnection.local_endpoint, self.preconnection.remote_endpoint, self.preconnection.transport_properties, self.preconnection.framer),
                    TransportInstance {
                        tcp_stream_instance: Some(incoming_conn.0),
                        udp_socket_instance: None,
                        quic_stream_instance: None,
                    }
                );
            }
        }
    }

}

impl<'a, T, U> Stream for Listener<'a, T, U> {
    type Item = Connection<'a, T, U>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Return any new transport instances as Connection objects

        let mut futures = FuturesUnordered::new();

        if self.tcp_listener.is_some() {
            futures.push(self.poll_tcp());
        }

        // Check for other listeners when implemented and add future to futures list

        if futures.is_empty() {
            return Poll::Pending;
        }

        let mut conn = None;

        task::block_on(async {
            conn = futures.next().await;
        });

        if conn.is_some() {
            return Poll::Ready(Some(conn.unwrap()));
        }

        return Poll::Pending;  
    }
}
