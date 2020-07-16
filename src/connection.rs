use crate::preconnection::Preconnection;

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

    pub async fn send(message_data: &[u8]) -> Result<(),()> {
        return Result::Ok(());
    }

    pub async fn receive() -> Result<(),()> {
        return Result::Ok(());
    }

    pub async fn close() -> Result<(),()> {
        return Result::Ok(());
    }

    pub fn abort() -> () { }
}