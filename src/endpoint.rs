
#[derive(Debug, Clone, Copy)]
pub struct LocalEndpoint<'a> {
    pub port: Option<u16>,
    pub address: Option<&'a str>,
    pub interface: Option<&'a str>,
}

impl<'a> LocalEndpoint<'a> {
    pub fn new() -> LocalEndpoint<'a> {
        LocalEndpoint {
            port: None,
            address: None,
            interface: None,
        }
    }

    pub fn with_port(&mut self, port: u16) -> () {
        self.port = Some(port);
    }

    pub fn with_address(&mut self, address: &'a str) -> () {
        self.address = Some(address);
    }

    pub fn with_interface(&mut self, interface: &'a str) -> () {
        self.interface = Some(interface);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RemoteEndpoint<'a> {
    pub port: Option<u16>,
    pub address: Option<&'a str>,
    pub host_name: Option<&'a str>,
    pub service: Option<&'a str>, // eg. https
}

impl<'a> RemoteEndpoint<'a> {
    pub fn new() -> RemoteEndpoint<'a> {
        RemoteEndpoint {
            port: None,
            address: None,
            host_name: None,
            service: None,
        }
    }

    pub fn with_port(&mut self, port: u16) -> () {
        self.port = Some(port);
    }

    pub fn with_address(&mut self, address: &'a str) -> () {
        self.address = Some(address);
    }

    pub fn with_host_name(&mut self, host_name: &'a str) -> () {
        self.host_name = Some(host_name);
    }

    pub fn with_service(&mut self, service: &'a str) -> () {
        self.service = Some(service);
    }
}