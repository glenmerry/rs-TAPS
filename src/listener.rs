use crate::preconnection::Preconnection;

pub struct Listener<'a> {
    preconnection: Preconnection<'a>,
}

impl<'a> Listener<'a> {
    pub fn new(
        preconnection: Preconnection<'a>,
    ) -> Listener<'a> {
        Listener {
            preconnection: preconnection,
        }
    }
}