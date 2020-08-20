use rs_taps::{
    error::TapsError,
    endpoint::{LocalEndpoint, RemoteEndpoint},
    transport_properties::TransportProperties,
    selection_properties::{SelectionProperty, PreferenceLevel},
    preconnection::Preconnection,
    message::Message,
};

use std::str;

use async_std::stream::StreamExt;

#[async_std::test]
async fn initiate_test() -> Result<(), TapsError> {
    let mut remote = RemoteEndpoint::new();
    remote.with_host_name("examp1e.net");
    remote.with_port(443);

    let mut tp = TransportProperties::default();
    tp.add(SelectionProperty::Reliability, PreferenceLevel::Require);
    tp.prefer(SelectionProperty::PreserveMsgBoundaries);

    let preconnection = Preconnection::new(None, Some(remote), Some(tp));

    let connection = preconnection.initiate().await;

    return match connection {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

#[async_std::test]
async fn listen_test() -> Result<(), TapsError> {
    let mut local = LocalEndpoint::new();
    local.with_address("127.0.0.1");
    local.with_port(12000);

    let tp = TransportProperties::default();
    
    let preconnection = Preconnection::new(Some(local), None, Some(tp));

    let mut listener = preconnection.listen().await?;
    listener.start().await?;

    while let Some(_) = listener.next().await {
        println!("new Connection");
        break;
    }

    Ok(())
}

#[async_std::test]
async fn send_test() -> Result<(), TapsError> {
    let mut remote = RemoteEndpoint::new();
    remote.with_host_name("example.com");
    remote.with_port(80);

    let mut tp = TransportProperties::default();
    tp.add(SelectionProperty::Reliability, PreferenceLevel::Require);
    tp.prefer(SelectionProperty::PreserveMsgBoundaries);

    let preconnection = Preconnection::new(None, Some(remote), Some(tp));

    let connection = preconnection.initiate().await?;

    let message = Message::new("GET / HTTP/1.1\r\nHost: example.com\r\n\r\n".as_bytes().to_vec(), None);

    connection.send(message).await?;

    let received = connection.receive().await?;
    println!("{:?}", str::from_utf8(&received.data));

    Ok(())
}