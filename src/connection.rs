use crate::error::TapsError;
use crate::preconnection::Preconnection;

#[derive(Debug)]
pub struct Connection<'a> {
    preconnection: Preconnection<'a>,
}

impl<'a> Connection<'a> {
    pub fn new(
        preconnection: Preconnection<'a>,
    ) -> Connection<'a> {
        Connection {
            preconnection: preconnection,
        }
    }

    pub async fn send(&self, message_data: &[u8]) -> Result<(),TapsError> {
        return Result::Ok(());
    }

    pub async fn receive(&self) -> Result<(),TapsError> {
        return Result::Ok(());
    }

    pub async fn close(&self) -> Result<(),TapsError> {
        return Result::Ok(());
    }

    pub fn abort(&self) -> () { }
}