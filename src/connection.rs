use crate::error::TapsError;
use crate::preconnection::Preconnection;
use crate::preconnection::TransportInstance;

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

    pub async fn send(&self, message_data: &[u8]) -> Result<(),TapsError> {
        todo!();
    }

    pub async fn receive(&self) -> Result<(),TapsError> {
        todo!();
    }

    pub async fn close(&self) -> Result<(),TapsError> {
        todo!();
    }

    pub fn abort(&self) -> () {
        todo!();
    }
}