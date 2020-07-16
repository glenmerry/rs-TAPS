use crate::endpoint::LocalEndpoint;
use crate::endpoint::RemoteEndpoint;
use crate::transport_properties::TransportProperties;
use crate::connection::Connection;
use crate::listener::Listener;

pub struct Preconnection<'a> {
    local_endpoint: Option<LocalEndpoint<'a>>,
    remote_endpoint: Option<RemoteEndpoint<'a>>,
    transport_properties: Option<TransportProperties>,
}

impl<'a> Preconnection<'a> {
    pub fn new(
        local_endpoint: Option<LocalEndpoint<'a>>, 
        remote_endpoint: Option<RemoteEndpoint<'a>>, 
        transport_properties: Option<TransportProperties>,
    ) -> Preconnection<'a> {
        Preconnection {
            local_endpoint: local_endpoint,
            remote_endpoint: remote_endpoint,
            transport_properties: transport_properties,
        }
    }

    pub async fn initiate(self) -> Connection<'a> {
        return Connection::new(self)
    }

    pub async fn listen(self) -> Listener<'a> {
        return Listener::new(self)
    }
}
