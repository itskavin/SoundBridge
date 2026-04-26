pub mod discovery;
pub mod realtime;

pub use discovery::{DiscoveryService, PeerInfo};
pub use realtime::{run_udp_client, run_udp_echo_server, RealtimeFrame, RealtimeStats};

use sb_core::{ClientHello, HandshakeError, ServerHello};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransportKind {
    Quic,
    RtpUdp,
}

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("invalid endpoint: {0}")]
    InvalidEndpoint(String),
    #[error(transparent)]
    Handshake(#[from] HandshakeError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub host: String,
    pub port: u16,
}

impl Endpoint {
    pub fn parse(raw: &str) -> Result<Self, TransportError> {
        let (host, port_str) = raw
            .split_once(':')
            .ok_or_else(|| TransportError::InvalidEndpoint(raw.to_string()))?;

        if host.trim().is_empty() {
            return Err(TransportError::InvalidEndpoint(raw.to_string()));
        }

        let port = port_str
            .parse::<u16>()
            .map_err(|_| TransportError::InvalidEndpoint(raw.to_string()))?;

        if port == 0 {
            return Err(TransportError::InvalidEndpoint(raw.to_string()));
        }

        Ok(Self {
            host: host.to_string(),
            port,
        })
    }
}

pub trait Transport {
    fn kind(&self) -> TransportKind;
    fn connect(&self, endpoint: &Endpoint) -> Result<(), TransportError>;
    fn establish_session(
        &self,
        endpoint: &Endpoint,
        hello: &ClientHello,
    ) -> Result<ServerHello, TransportError>;
}

#[derive(Debug, Default)]
pub struct QuicTransport;

impl Transport for QuicTransport {
    fn kind(&self) -> TransportKind {
        TransportKind::Quic
    }

    fn connect(&self, endpoint: &Endpoint) -> Result<(), TransportError> {
        if endpoint.host.trim().is_empty() || endpoint.port == 0 {
            return Err(TransportError::InvalidEndpoint(format!(
                "{}:{}",
                endpoint.host, endpoint.port
            )));
        }
        Ok(())
    }

    fn establish_session(
        &self,
        endpoint: &Endpoint,
        hello: &ClientHello,
    ) -> Result<ServerHello, TransportError> {
        self.connect(endpoint)?;
        hello.validate()?;

        Ok(ServerHello {
            session_id: format!("{}-{}", hello.client_id, endpoint.port),
            accepted_transport: "quic".to_string(),
            target_buffer_ms: hello.profile.target_buffer_ms(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Endpoint, QuicTransport, Transport};
    use sb_core::{ClientHello, Platform, QualityProfile};

    #[test]
    fn endpoint_parser_accepts_host_and_port() {
        let endpoint = Endpoint::parse("127.0.0.1:7000").expect("valid endpoint");
        assert_eq!(endpoint.host, "127.0.0.1");
        assert_eq!(endpoint.port, 7000);
    }

    #[test]
    fn endpoint_parser_rejects_invalid_input() {
        assert!(Endpoint::parse("127.0.0.1").is_err());
        assert!(Endpoint::parse(" :7000").is_err());
        assert!(Endpoint::parse("localhost:0").is_err());
    }

    #[test]
    fn quic_transport_establishes_session_for_valid_hello() {
        let endpoint = Endpoint::parse("localhost:9000").expect("valid endpoint");
        let hello = ClientHello {
            client_id: "desktop-main".to_string(),
            platform: Platform::Windows,
            preferred_transport: "quic".to_string(),
            profile: QualityProfile::Voice,
        };

        let transport = QuicTransport;
        let server = transport
            .establish_session(&endpoint, &hello)
            .expect("session should establish");

        assert_eq!(server.accepted_transport, "quic");
        assert_eq!(server.target_buffer_ms, 60);
        assert!(server.session_id.starts_with("desktop-main-"));
    }
}
