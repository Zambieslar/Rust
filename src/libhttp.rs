use openssl::{self, aes, base64, ssl::{self, SslConnector, SslMethod, SslStream}};
use std::{fmt::Error, net::{self, TcpStream}};

pub fn ssl_connect() -> SslStream<TcpStream> {
    let ssl_connector = SslConnector::builder(SslMethod::tls_client())
        .unwrap()
        .build();
    let stream = TcpStream::connect("google.com:443").unwrap();
    let mut stream = ssl_connector.connect("google.com", stream).unwrap();
    stream
}
