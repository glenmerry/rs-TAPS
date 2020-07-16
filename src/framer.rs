use crate::connection::Connection;

pub trait Framer {

    // use async-trait crate so these functions can be async?

    fn start(&mut self, connection: Connection) {

    }

    fn stop(&mut self, connection: Connection) {

    }

    fn new_sent_message(&mut self, connection: Connection, message_data: &mut [u8]) {

    }

    fn handle_received_data(&mut self, connection: Connection) {

    }


}