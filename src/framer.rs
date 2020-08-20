use crate::connection::Connection;
use crate::message::Message;

pub trait Framer {
    fn start(&self, connection: &Connection);
    fn stop(&self, connection: &Connection);
    fn new_sent_message(&self, connection: &Connection, message: & dyn Message) -> Vec<u8>;
    fn handle_received_data(&self, connection: &Connection);
}


pub struct HttpRequestFramer;

impl Framer for HttpRequestFramer {
    fn start(&self, _connection: &Connection) {

    }

    fn stop(&self, _connection: &Connection) {

    }

    fn new_sent_message(&self, _connection: &Connection, message: & dyn Message) -> Vec<u8> {
        todo!();
    }

    fn handle_received_data(&self, _connection: &Connection) {
        // create http_response message

    }
}
