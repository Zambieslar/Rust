use openssl::{self, error::ErrorStack, ssl::{HandshakeError, SslConnector, SslMethod, SslStream}};
use std::net::TcpStream;

pub fn ssl_connect(server: &str, badge_data: &str) -> Result<(SslStream<_>, String), HandshakeError<TcpStream>> {
    let req = format!("{{\"data\": [{{\"type\": \"rawBadgeData\", \"attributes\": {{\"value\"\"{}\"}}}}]}}", badge_data);
    let ssl_context = SslConnector::builder(SslMethod::tls_client()).unwrap().build();
    let stream = TcpStream::connect(format!("{}:31988", server)).unwrap();
    let ssl_connector = ssl_context.connect(server, stream);
    Ok((ssl_connector, req.to_string()))
}
