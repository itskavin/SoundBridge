use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AudioMode {
    Speaker,
    Microphone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub sample_rate_hz: u32,
    pub channels: u8,
    pub mode: AudioMode,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate_hz: 48_000,
            channels: 2,
            mode: AudioMode::Speaker,
        }
    }
}
