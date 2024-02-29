use openssl::{self, ssl::{SslConnector, SslMethod, SslStream}};
use std::{net::TcpStream, time::Duration};

//enum StreamError {
//    TlsHandshakeError(HandshakeError<TcpStream>),
//    ConnectFailure(Shutdown),
//    ReadTimeOut(std::io::Error)
//}
//
//impl std::error::Error for StreamError {}
//
//impl fmt::Display for StreamError {
//    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match self {
//            StreamError::TlsHandshakeError(e) => {write!(f, "Handshake error")}
//            StreamError::ConnectFailure(e) => {write!(f, "Unable to connect")}
//        }
//    }
//}
//
//impl Debug for StreamError {
//    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match self {
//            StreamError::TlsHandshakeError(e) => {write!(f, "Handshake error")}
//            StreamError::ConnectFailure(e) => {write!(f, "Unable to connect")}
//        }
//    }
//}


pub fn ssl_connect(server: &str) -> Result<SslStream<TcpStream>, Box<dyn std::error::Error>> {
    let ssl_context = SslConnector::builder(SslMethod::tls_client()).unwrap().build();
    let stream = TcpStream::connect(format!("{}:443", server))?;
    let mut ssl_connector = ssl_context.connect(server, stream)?;
    Ok(ssl_connector)
}
