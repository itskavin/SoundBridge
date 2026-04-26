use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Idle,
    Discovering,
    Connecting,
    Connected,
    Reconnecting,
    Failed,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LifecycleError {
    #[error("invalid transition from {from:?} to {to:?}")]
    InvalidTransition {
        from: ConnectionState,
        to: ConnectionState,
    },
}

#[derive(Debug, Clone)]
pub struct SessionLifecycle {
    state: ConnectionState,
}

impl Default for SessionLifecycle {
    fn default() -> Self {
        Self {
            state: ConnectionState::Idle,
        }
    }
}

impl SessionLifecycle {
    pub fn state(&self) -> ConnectionState {
        self.state
    }

    pub fn start_discovery(&mut self) -> Result<(), LifecycleError> {
        self.transition(ConnectionState::Discovering)
    }

    pub fn start_connect(&mut self) -> Result<(), LifecycleError> {
        self.transition(ConnectionState::Connecting)
    }

    pub fn mark_connected(&mut self) -> Result<(), LifecycleError> {
        self.transition(ConnectionState::Connected)
    }

    pub fn start_reconnect(&mut self) -> Result<(), LifecycleError> {
        self.transition(ConnectionState::Reconnecting)
    }

    pub fn mark_failed(&mut self) -> Result<(), LifecycleError> {
        self.transition(ConnectionState::Failed)
    }

    pub fn reset(&mut self) -> Result<(), LifecycleError> {
        self.transition(ConnectionState::Idle)
    }

    fn transition(&mut self, next: ConnectionState) -> Result<(), LifecycleError> {
        let valid = matches!(
            (self.state, next),
            (ConnectionState::Idle, ConnectionState::Discovering)
                | (ConnectionState::Idle, ConnectionState::Connecting)
                | (ConnectionState::Discovering, ConnectionState::Connecting)
                | (ConnectionState::Connecting, ConnectionState::Connected)
                | (ConnectionState::Connecting, ConnectionState::Failed)
                | (ConnectionState::Connected, ConnectionState::Reconnecting)
                | (ConnectionState::Connected, ConnectionState::Idle)
                | (ConnectionState::Reconnecting, ConnectionState::Connected)
                | (ConnectionState::Reconnecting, ConnectionState::Failed)
                | (ConnectionState::Failed, ConnectionState::Idle)
        );

        if !valid {
            return Err(LifecycleError::InvalidTransition {
                from: self.state,
                to: next,
            });
        }

        self.state = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ConnectionState, SessionLifecycle};

    #[test]
    fn successful_connection_flow() {
        let mut lifecycle = SessionLifecycle::default();
        lifecycle.start_discovery().expect("discover should start");
        lifecycle.start_connect().expect("connect should start");
        lifecycle.mark_connected().expect("should connect");
        assert_eq!(lifecycle.state(), ConnectionState::Connected);
    }

    #[test]
    fn failed_and_recovery_flow() {
        let mut lifecycle = SessionLifecycle::default();
        lifecycle.start_connect().expect("connect should start");
        lifecycle.mark_failed().expect("can fail from connecting");
        lifecycle.reset().expect("failed can reset to idle");
        assert_eq!(lifecycle.state(), ConnectionState::Idle);
    }

    #[test]
    fn invalid_transition_is_rejected() {
        let mut lifecycle = SessionLifecycle::default();
        let err = lifecycle
            .mark_connected()
            .expect_err("idle should not connect directly");
        assert_eq!(err.to_string(), "invalid transition from Idle to Connected");
    }
}
