extern crate pnet;

use crate::error::TapsError;
use crate::endpoint::LocalEndpoint;
use crate::endpoint::RemoteEndpoint;
use crate::transport_properties::TransportProperties;
use crate::connection::Connection;
use crate::listener::Listener;
use crate::selection_properties;
use crate::selection_properties::ServiceLevel;
use crate::selection_properties::PreferenceLevel;

use std::collections::HashMap;
use std::collections::HashSet;

use async_std::net::{SocketAddr, ToSocketAddrs};
use pnet::datalink;

#[derive(Debug)]
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

    pub async fn initiate(self) -> Result<Connection<'a>, TapsError> {

        // Ensure sufficient remote endpoint parameters have been supplied for Connection establishment
        if self.remote_endpoint.is_none() {
            return Err(TapsError::RemoteEndpointNotProvided);
        } 
        if self.remote_endpoint.as_ref().unwrap().port.is_none() {
            return Err(TapsError::RemoteEndpointPortNotProvided);
        } 
        if self.remote_endpoint.as_ref().unwrap().address.is_none() && self.remote_endpoint.as_ref().unwrap().host_name.is_none() {
            return Err(TapsError::RemoteEndpointAddressAndHostNameBothNotProvided);
        }

        // CANDIDATE GATHERING

        // Gather remote endpoint candidates

        let port = self.remote_endpoint.as_ref().unwrap().port.as_ref().unwrap();
        let mut candidate_remote_addrs = HashSet::new();

        // IP address provided in remote endpoint
        if self.remote_endpoint.as_ref().unwrap().address.is_some() {
            let addr = self.remote_endpoint.as_ref().unwrap().address.as_ref().unwrap();
            let from_addr = format!("{}:{}", addr, port).to_socket_addrs().await?;
            for a in from_addr {
                candidate_remote_addrs.insert(a);
            }
        }

        // Host name provided in remote endpoint
        if self.remote_endpoint.as_ref().unwrap().host_name.is_some() {
            let host_name = self.remote_endpoint.as_ref().unwrap().host_name.as_ref().unwrap();
            let from_host_name = format!("{}:{}", host_name, port).to_socket_addrs().await?;
            for a in from_host_name {
                candidate_remote_addrs.insert(a);
            }
        }

        println!("Candidate Remote Endpoint Addresses: {:?}", candidate_remote_addrs);

        // Gather local endpoint candidates - local endpoint is optional for initiating Connection
        let mut candidate_local_addrs: std::collections::HashSet<SocketAddr> = HashSet::new();
        if self.local_endpoint.is_some() {
            todo!();
            // for iface in datalink::interfaces() {
            // }
        }

        // Gather protocol stack candidates
        
        // Get available protocol stacks on system 
        let candidate_protocols = selection_properties::get_supported_protocols();
        let mut candidate_protocol_ranks = HashMap::new();
        for candidate_protocols in candidate_protocols.keys() {
            candidate_protocol_ranks.insert(candidate_protocols, 0);
        }

        // Calculate ranks for each of the candidate protocol stacks, depending on their Service Level for each of
        // the Selection Properties Preference Levels.
        'outer: for (candidate, service_levels) in candidate_protocols.iter() {
            for (property, &preference_level) in self.transport_properties.unwrap().selection_properties.iter() {
                let property_service_level = service_levels[property];

                match preference_level {
                    PreferenceLevel::Require => {
                        // If a protocol stack does not provide a required property,
                        // remove it from the list of candidates and continue to next protocol stack
                        if let ServiceLevel::NotProvided = property_service_level {
                            candidate_protocol_ranks.remove(candidate);
                            continue 'outer;
                        }
                    },

                    PreferenceLevel::Prefer => {
                        // Increase rank of protocol stacks which provide or have optional provision of a preferred property
                        if property_service_level == ServiceLevel::Provided || property_service_level == ServiceLevel::Optional {
                            *candidate_protocol_ranks.get_mut(candidate).unwrap() += 1;
                        } 
                    },

                    PreferenceLevel::Ignore => {
                        // Preference is to be ignored, do nothing
                    },

                    PreferenceLevel::Avoid => {
                        // Increase rank of protocol stacks which do not provide a property which is to be avoided
                        if let ServiceLevel::NotProvided = property_service_level {
                            *candidate_protocol_ranks.get_mut(candidate).unwrap() += 1;
                        }
                    },

                    PreferenceLevel::Prohibit => {
                        // If a protocol stack has a prohibited property, remove it, continue to next protocol stack
                        if property_service_level != ServiceLevel::NotProvided {
                            candidate_protocol_ranks.remove(candidate);
                            continue 'outer;
                        }
                    }
                }
            }
        }

        if candidate_protocol_ranks.len() == 0 {
            return Err(TapsError::NoCompatibleProtocolStacks);
        }

        for (protocol, rank) in candidate_protocol_ranks.iter() {
            println!("Candidate protocol stack: {} has rank {}", protocol, rank); 
        }

        // Build candidate set for racing based on combinations of protocol stacks and local and remote IP addresses

        let mut candidates: std::vec::Vec<(SocketAddr, Option<SocketAddr>, &str)> = vec![];

        // Local endpoint supplied - include in candidate combinations
        if !candidate_local_addrs.is_empty() {
            for remote_addr in candidate_remote_addrs {
                for local_addr in &candidate_local_addrs {
                    for protocol in &candidate_protocol_ranks {
                        candidates.push((remote_addr, Some(*local_addr), protocol.0));
                    }
                }
            }

        // No local endpoints supplied - do not include in candidate combinations
        } else {
            for remote_addr in candidate_remote_addrs {
                for protocol in &candidate_protocol_ranks {
                    candidates.push((remote_addr, None, protocol.0));
                }
            }
        }

        println!("{:?}", candidates);
        
        // CANDIDATE RACING

        // Use delayed racing with connection attempts launched in parallel with a delay between each category, in this order:

        // * IPv6 candidates with highest ranked protocol stack (if rank is a tie with another protocol then break randomly)
        // * IPv4 candidates with highest ranked protocol
        // * IPv6 candidates with 2nd highest ranked protocol
        // * IPv4 candidates with 2nd highest ranked protocol
        // and so on...

        // As soon as a connection is successfully established, candidate racing will stop and the successful connection will be returned

        return Ok(Connection::new(self));
    }

    pub async fn listen(self) -> Result<Listener<'a>, TapsError> {

        if self.local_endpoint.is_none() {
            return Err(TapsError::LocalEndpointNotProvided);
        }

        todo!();

        return Ok(Listener::new(self))
    }
}
