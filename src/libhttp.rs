use openssl::{self, error::ErrorStack, ssl::{HandshakeError, SslConnector, SslMethod, SslStream}};
use std::net::TcpStream;

pub fn ssl_connect(server: &str) -> Result<SslStream<TcpStream>, HandshakeError<TcpStream>> {
    let ssl_context = SslConnector::builder(SslMethod::tls_client()).unwrap().build();
    let stream = TcpStream::connect(format!("{}:31988", server)).unwrap();
    let ssl_connector = ssl_context.connect(server, stream).unwrap();
    Ok(ssl_connector)
}
