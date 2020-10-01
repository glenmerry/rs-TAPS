use crate::message_context::MessageContext;

#[derive(Debug, Clone)]
pub struct Message<T> {
    pub data: T,
    message_context: Option<MessageContext>,
}

impl<T> Message<T> {
    pub fn new(data: T, message_context: Option<MessageContext>) -> Message<T> {
        Message {
            data: data,
            message_context: message_context,
        }
    }
}
