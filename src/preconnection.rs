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
use std::time::Duration;

use async_std::{
    prelude::*,
    task,
    net::{SocketAddr, ToSocketAddrs, TcpStream, UdpSocket},
};
use futures::stream::FuturesUnordered;
use itertools::interleave;

#[derive(Debug)]
struct TransportInstance {
    tcp_stream_instance: Option<TcpStream>,
    udp_socket_instance: Option<UdpSocket>,
    quic_stream_instance: Option<()>,
}

#[derive(Debug)]
pub struct Preconnection<'a> {
    local_endpoint: Option<LocalEndpoint<'a>>,
    remote_endpoint: Option<RemoteEndpoint<'a>>,
    transport_properties: Option<TransportProperties>,
    transport_instance: Option<TransportInstance>,
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
            transport_instance: None,
        }
    }

    pub async fn initiate(mut self) -> Result<Connection<'a>, TapsError> {

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

        // If no Transport Properties provided, use default Transport Properties
        if self.transport_properties.is_none() {
            self.transport_properties = Some(TransportProperties::default());
        }

        {
            // Gather candidate connections
            let candidates = self.gather_candidates().await;

            let candidates = match candidates {
                Ok(c) => c,
                Err(e) => return Err(e),
            };

            // Race gatherered candidates

            // Use delayed racing with connection attempts launched in parallel with a delay between each.

            let mut futures = FuturesUnordered::new();

            let mut delay = Duration::from_millis(0);

            for candidate in candidates {
                futures.push(attempt_connection(candidate, delay));
                delay = delay + Duration::from_millis(250);
            }

            task::block_on(async {
                while let Some(transport_instance) = futures.next().await {
                    let transport_instance = match transport_instance {
                        Ok(transport_instance) => {
                            self.transport_instance = Some(transport_instance);
                            if self.transport_instance.as_ref().unwrap().tcp_stream_instance.is_some() {
                                println!("Connected using TCP");
                            } else if self.transport_instance.as_ref().unwrap().udp_socket_instance.is_some() {
                                println!("Connected using UDP");
                            }
                            
                            return Ok(Connection::new(self));
                        },
                        Err(_) => println!("Connection attempt failed"),
                    };
                }

                return Err::<Connection<'a>, TapsError>(TapsError::NoCandidateSucceeded);
            })
        }
    }

    // Perform candidate gathering for Connection initiation
    async fn gather_candidates(&self) -> Result<std::vec::Vec<(SocketAddr, Option<SocketAddr>, &'static str)>, TapsError> {

        // Gather remote endpoint candidates
        let remote_port = self.remote_endpoint.as_ref().unwrap().port.as_ref().unwrap();
        let mut candidate_remote_addrs = HashSet::new();

        // IP address provided in remote endpoint
        if self.remote_endpoint.as_ref().unwrap().address.is_some() {
            let remote_addr = self.remote_endpoint.as_ref().unwrap().address.as_ref().unwrap();
            let from_addr = format!("{}:{}", remote_addr, remote_port).to_socket_addrs().await?;
            for a in from_addr {
                candidate_remote_addrs.insert(a);
            }
        }

        // Host name provided in remote endpoint - DNS lookup performed here
        if self.remote_endpoint.as_ref().unwrap().host_name.is_some() {
            let host_name = self.remote_endpoint.as_ref().unwrap().host_name.as_ref().unwrap();
            let from_host_name = format!("{}:{}", host_name, remote_port).to_socket_addrs().await?;
            for a in from_host_name {
                candidate_remote_addrs.insert(a);
            }
        }

        println!("Candidate Remote Endpoint Addresses: {:?}", candidate_remote_addrs);

        // Gather local endpoint candidates - local endpoint is optional for initiating Connection
        let mut candidate_local_addrs = HashSet::new();

        if self.local_endpoint.is_some() {
            let local_port = self.local_endpoint.as_ref().unwrap().port.as_ref().unwrap();
            let local_addr = self.local_endpoint.as_ref().unwrap().address.as_ref().unwrap();
            let local_socket_addr = format!("{}:{}", local_addr, local_port).to_socket_addrs().await?.next().unwrap();
            candidate_local_addrs.insert(local_socket_addr);
        }

        // Gather protocol stack candidates

        // Get the candidate protocol stacks available on the system, along with their ranks for connection racing
        let candidate_protocol_ranks = self.calculate_candidate_protocol_ranks();

        let candidate_protocol_ranks = match candidate_protocol_ranks {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        for (protocol, rank) in candidate_protocol_ranks.iter() {
            println!("Candidate protocol stack: {} has rank {}", protocol, rank); 
        }

        let mut candidate_protocol_ranks_sorted: Vec<_> = candidate_protocol_ranks.iter().collect();
        candidate_protocol_ranks_sorted.sort_by(|a, b| a.1.cmp(b.1).reverse());

        // Build candidate set for racing based on combinations of protocol stacks and local and remote IP addresses

        let mut candidates: std::vec::Vec<(SocketAddr, Option<SocketAddr>, &str)> = vec![];

        for protocol in candidate_protocol_ranks_sorted {
            // Get IPv6 and IPv4 candidates for this protocol
            let mut ipv6_candidates: std::vec::Vec<(SocketAddr, Option<SocketAddr>, &str)> = vec![];
            let mut ipv4_candidates: std::vec::Vec<(SocketAddr, Option<SocketAddr>, &str)> = vec![];

            for remote_addr in &candidate_remote_addrs {
                let candidate_vec_ref;
                if remote_addr.is_ipv6() {
                    candidate_vec_ref = &mut ipv6_candidates;
                } else {
                    candidate_vec_ref = &mut ipv4_candidates;
                }

                // Local endpoint supplied - include in candidate combinations
                if !candidate_local_addrs.is_empty() {
                    for local_addr in &candidate_local_addrs {
                        candidate_vec_ref.push((*remote_addr, Some(*local_addr), protocol.0));
                    }

                // No local endpoints supplied - do not include in candidate combinations
                } else {
                    candidate_vec_ref.push((*remote_addr, None, protocol.0));
                }
            }

            // Interleave IPv6 and IPv4 candidates for protocol according to Happy Eyeballs algorithm 
            candidates.extend(interleave(ipv6_candidates, ipv4_candidates));
        }

        println!("\nFinal candidates: {:?}", candidates);

        return Ok(candidates);
    }

    fn calculate_candidate_protocol_ranks(&self) -> Result<HashMap<&'static str, u8>, TapsError> {

        // Get available protocol stacks on system 
        let candidate_protocols = selection_properties::get_supported_protocols();
    
        // Calculate ranks for each of the candidate protocol stacks, depending on their Service Level for each of
        // the Selection Properties Preference Levels.
        let mut candidate_protocol_ranks = HashMap::<&str, u8>::new();
        for candidate_protocols in candidate_protocols.keys() {
            candidate_protocol_ranks.insert(candidate_protocols, 0);
        }
    
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
        return Ok(candidate_protocol_ranks);
    }

    pub async fn listen(self) -> Result<Listener<'a>, TapsError> {

        if self.local_endpoint.is_none() {
            return Err(TapsError::LocalEndpointNotProvided);
        }

        todo!();

        return Ok(Listener::new(self))
    }
}

async fn attempt_connection(candidate: (SocketAddr, Option<SocketAddr>, &'static str), delay: Duration) -> Result<TransportInstance, TapsError> {
    task::sleep(delay).await;

    let (remote_addr, local_addr, protocol) = candidate;

    match protocol {
        "tcp" => return connect_tcp(remote_addr, local_addr).await,
        // "quic" => return connect_quic(addr).await,
        "udp" => return connect_udp(remote_addr, local_addr).await,
        _ => return Err(TapsError::ProtocolNotSupported),
    }
}

async fn connect_tcp(remote_addr: SocketAddr, local_addr: Option<SocketAddr>) -> Result<TransportInstance, TapsError> {
    println!("Attempting TCP connection to: {:?}", remote_addr);

    let stream = TcpStream::connect(remote_addr).await;
    let stream = match stream {
        Ok(stream) => stream,
        Err(_) => {println!("TCP connection attempt failed"); return Err(TapsError::ConnectionAttemptFailed)},
    };

    return Ok(TransportInstance {
        tcp_stream_instance: Some(stream),
        udp_socket_instance: None,
        quic_stream_instance: None,
    });
}

async fn connect_udp(remote_addr: SocketAddr, local_addr: Option<SocketAddr>) -> Result<TransportInstance, TapsError> {
    if local_addr.is_none() {
        return Err(TapsError::ConnectionAttemptFailed);
    }

    println!("Attempting to create UDP socket bound to local address: {:?}, connected to remote address: {:?}", local_addr, remote_addr);

    let socket = UdpSocket::bind(local_addr.unwrap()).await;
    let socket = match socket {
        Ok(socket) => socket,
        Err(_) => {println!("UDP connection attempt failed"); return Err(TapsError::ConnectionAttemptFailed)},
    };
    
    let connect_result = socket.connect(remote_addr).await;
    match connect_result {
        Ok(_) => (),
        Err(_) => {println!("UDP connection attempt failed"); return Err(TapsError::ConnectionAttemptFailed)},
    };

    return Ok(TransportInstance {
        tcp_stream_instance: None,
        udp_socket_instance: Some(socket),
        quic_stream_instance: None,
    });
}
