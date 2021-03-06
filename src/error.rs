use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TapsError {
    Io(io::Error),
    RemoteEndpointNotProvided,
    LocalEndpointNotProvided,
    RemoteEndpointPortNotProvided,
    RemoteEndpointAddressAndHostNameBothNotProvided,
    NoCompatibleProtocolStacks,
    ProtocolNotSupported,
    ConnectionAttemptFailed, // TODO: allowing embedding specific details of connection attempt failure 
    NoCandidateSucceeded,
    MessageSendFailed,
    MessageReceiveFailed,
}

impl fmt::Display for TapsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TapsError::Io(ref err) => err.fmt(f),
            TapsError::RemoteEndpointNotProvided                       => write!(f, "No remote endpoint was provided. \
                                                                                    A remote endpoint must be provided in a preconnection for \
                                                                                    Connection initiation to take place."),
            TapsError::LocalEndpointNotProvided                        => write!(f, "No local endpoint was provided. \
                                                                                     A local endpoint must be supplied to listen for incoming Connections."),
            TapsError::RemoteEndpointPortNotProvided                   => write!(f, "No port was provided in the remote endpoint. \
                                                                                    A port must be provided in the remote endpoint for \
                                                                                    Connection initiation to take place."),
            TapsError::RemoteEndpointAddressAndHostNameBothNotProvided => write!(f, "Both address and host name were not provided in the remote endpoint. \
                                                                                    At least one of the address and host name must be provided in the remote endpoint for \
                                                                                    Connection initiation to take place."),
            TapsError::NoCompatibleProtocolStacks                      => write!(f, "After performing candidate gathering, no protocol stacks \
                                                                                     were found that satisfy the provided Transport Properties. \
                                                                                     Therefore, Connection initiation cannot take place."),
            TapsError::ProtocolNotSupported                            => write!(f, "Attempt was made to connect using a protocol stack which is not supported by rs_taps."),
            TapsError::ConnectionAttemptFailed                         => write!(f, "Establishing a particular candidate connection during connection racing failed. \
                                                                                     Other candidate connections will be attempted if available."),
            TapsError::NoCandidateSucceeded                            => write!(f, "No candidate connection handshake completed successfully. \
                                                                                     Therefore, Connection ititiation was unsuccessful"),
            TapsError::MessageSendFailed                               => write!(f, "Error sending message"),
            TapsError::MessageReceiveFailed                            => write!(f, "Error receiving message"),
        }
    }
}

impl Error for TapsError { }

impl From<io::Error> for TapsError {
    fn from(err: io::Error) -> TapsError {
        TapsError::Io(err)
    }
}
