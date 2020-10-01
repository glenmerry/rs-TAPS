use rs_taps::{
    error::TapsError,
    endpoint::{LocalEndpoint, RemoteEndpoint},
    transport_properties::TransportProperties,
    selection_properties::{SelectionProperty, PreferenceLevel},
    preconnection::Preconnection,
    message::Message,
    framer::HttpClientFramer,
};

use async_std::stream::StreamExt;

use http::{Request, Response};

#[async_std::test]
async fn initiate_test() -> Result<(), TapsError> {
    let mut remote = RemoteEndpoint::new();
    remote.with_host_name("examp1e.net");
    remote.with_port(443);

    let mut tp = TransportProperties::default();
    tp.add(SelectionProperty::Reliability, PreferenceLevel::Require);
    tp.prefer(SelectionProperty::PreserveMsgBoundaries);

    let preconnection = Preconnection::<Request<()>, Response<()>>::new(
        None, 
        Some(remote), 
        Some(tp),
        &HttpClientFramer{});

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
    
    let preconnection = Preconnection::<Request<()>, Response<()>>::new(
        Some(local),
        None, 
        Some(tp),
        &HttpClientFramer{});

    let mut listener = preconnection.listen().await?;
    listener.start().await?;

    while let Some(_) = listener.next().await {
        println!("new Connection");
        break;
    }

    Ok(())
}

#[async_std::test]
async fn send_receive_test() -> Result<(), TapsError> {
    let mut remote = RemoteEndpoint::new();
    remote.with_host_name("gla.ac.uk");
    remote.with_port(80);
    let tp = TransportProperties::default();

    let preconnection = Preconnection::<Request<()>, Response<()>>::new(
        None, 
        Some(remote), 
        Some(tp), 
        &HttpClientFramer{});

    let mut connection = preconnection.initiate().await?;

    let request = Request::builder()
        .method("GET")
        .uri("www.gla.ac.uk")
        .body(())
        .unwrap();

    let message = Message::<Request<()>>::new(request, None);

    connection.send(message).await?;

    let received_message = connection.receive().await?;
    println!("Received Message: {:?}", &received_message);

    Ok(())
}