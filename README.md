# rs-TAPS: A Rust implementation of the IETF Transport Services API

The IETF's [TAPS API](https://datatracker.ietf.org/doc/draft-ietf-taps-arch) is an attempt to produce a new, modern interface for network programming, replacing Berkeley sockets. It's design decouples applications from specific transport protocols (such as TCP, UDP). Instead, they specify only which transport services they require (for example, reliable data transfer or congestion control). Further benefits over the legacy approach include typed data transfer, asynchronous operation and support for connection racing. 

The rs-TAPS library is an implementation of the TAPS API in the Rust programming language. This project is still in an early stage of development but provides a proof of concept of the TAPS system. The [async-std](https://github.com/async-rs/async-std) library is used to provide asynchronous functionality and is necessary to call the library's asynchonous functions.

## Use

To use the rs-TAPS library in your project, add the following dependency to the Cargo.toml file:

```toml
[dependencies]
rs_taps = { git = "https://github.com/glenmerry/rs-TAPS.git" }
```

For an example of using the API, see the test definitions in tests/tests.rs.

## Testing

To run the tests, use the command:

```
cargo run
```
