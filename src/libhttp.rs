use openssl::{self, aes, base64, ssl::{self, Ssl, SslConnector, SslConnectorBuilder, SslContextBuilder, SslMethod, SslStream, SslVerifyMode}};
use std::{fmt::Error, net::{self, TcpStream}, ops::Index};

pub fn ssl_connect(server: &str, badge_data: &str) -> (SslStream<TcpStream>, String) {
    let req = format!("{{\"data\": [{{\"type\": \"rawBadgeData\", \"attributes\": {{\"value\"\"{}\"}}}}]}}", badge_data);
    let ssl_context = SslConnector::builder(SslMethod::tls_client()).unwrap().build();
    let stream = TcpStream::connect(format!("{}:31988", server)).unwrap();
    let ssl_connector = ssl_context.connect(server, stream).unwrap();
    (ssl_connector, req.to_string())
}
