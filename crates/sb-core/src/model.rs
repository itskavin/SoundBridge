use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum QualityProfile {
    Voice,
    Music,
    UltraLowLatency,
}

impl QualityProfile {
    pub fn target_buffer_ms(self) -> u16 {
        match self {
            QualityProfile::Voice => 60,
            QualityProfile::Music => 90,
            QualityProfile::UltraLowLatency => 30,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Platform {
    Windows,
    MacOs,
    Linux,
    Android,
    Ios,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientHello {
    pub client_id: String,
    pub platform: Platform,
    pub preferred_transport: String,
    pub profile: QualityProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHello {
    pub session_id: String,
    pub accepted_transport: String,
    pub target_buffer_ms: u16,
}

#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("client id must not be empty")]
    EmptyClientId,
    #[error("unsupported transport: {0}")]
    UnsupportedTransport(String),
}

impl ClientHello {
    pub fn validate(&self) -> Result<(), HandshakeError> {
        if self.client_id.trim().is_empty() {
            return Err(HandshakeError::EmptyClientId);
        }

        if self.preferred_transport != "quic" && self.preferred_transport != "rtp_udp" {
            return Err(HandshakeError::UnsupportedTransport(
                self.preferred_transport.clone(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub profile: QualityProfile,
}

impl SessionInfo {
    pub fn new(id: &str, profile: QualityProfile) -> Self {
        Self {
            id: id.to_string(),
            profile,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ClientHello, HandshakeError, Platform, QualityProfile};

    #[test]
    fn validates_supported_handshake() {
        let hello = ClientHello {
            client_id: "desktop-1".to_string(),
            platform: Platform::Windows,
            preferred_transport: "quic".to_string(),
            profile: QualityProfile::Voice,
        };

        assert!(hello.validate().is_ok());
    }

    #[test]
    fn rejects_empty_client_id() {
        let hello = ClientHello {
            client_id: " ".to_string(),
            platform: Platform::Linux,
            preferred_transport: "quic".to_string(),
            profile: QualityProfile::Music,
        };

        let err = hello.validate().expect_err("empty id should fail");
        assert!(matches!(err, HandshakeError::EmptyClientId));
    }

    #[test]
    fn rejects_unknown_transport() {
        let hello = ClientHello {
            client_id: "mobile-a".to_string(),
            platform: Platform::Android,
            preferred_transport: "tcp".to_string(),
            profile: QualityProfile::UltraLowLatency,
        };

        let err = hello
            .validate()
            .expect_err("unsupported transport should fail");
        assert!(matches!(err, HandshakeError::UnsupportedTransport(_)));
    }
}
