mod endpoint;
mod preconnection;
mod transport_properties;
mod selection_properties;
mod connection;
mod listener;
mod framer;
mod error;

#[async_std::test]
async fn test() -> Result<(), error::TapsError> {
    let mut remote = endpoint::RemoteEndpoint::new();
    // ep.with_host_name("apple.com");
    remote.with_address("127.0.0.1");
    remote.with_port(13370);

    let mut local = endpoint::LocalEndpoint::new();
    local.with_address("127.0.0.1");
    local.with_port(12000);

    let mut tp = transport_properties::TransportProperties::default();
    // tp.add(selection_properties::SelectionProperty::Reliability, selection_properties::PreferenceLevel::Require);
    // tp.prefer(selection_properties::SelectionProperty::PreserveMsgBoundaries);

    let preconnection = preconnection::Preconnection::new(Some(local), Some(remote), Some(tp));

    let connection = preconnection.initiate().await;

    let connection = match connection {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    Ok(())
}