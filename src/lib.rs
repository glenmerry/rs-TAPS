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
    let mut ep = endpoint::RemoteEndpoint::new();
    // ep.with_host_name("apple.com");
    ep.with_address("127.0.0.1");
    ep.with_port(13370);

    let mut tp = transport_properties::TransportProperties::default();
    tp.add(selection_properties::SelectionProperty::Reliability, selection_properties::PreferenceLevel::Require);
    tp.prefer(selection_properties::SelectionProperty::PreserveMsgBoundaries);

    let preconnection = preconnection::Preconnection::new(None, Some(ep), Some(tp));

    let connection = preconnection.initiate().await;

    let connection = match connection {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    Ok(())
}