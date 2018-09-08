use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::{commands, state::State, MockHSM};
use adapters::{Adapter, AdapterError, AdapterErrorKind::ConnectionFailed};
use commands::CommandType;
use securechannel::CommandMessage;

/// A mocked connection to the MockHSM
pub struct MockAdapter(Arc<Mutex<State>>);

impl Adapter for MockAdapter {
    type Config = MockHSM;

    /// Create a new adapter with a clone of the MockHSM state
    fn open(hsm: &MockHSM) -> Result<Self, AdapterError> {
        Ok(MockAdapter(hsm.0.clone()))
    }

    /// Rust never sleeps
    fn healthcheck(&self) -> Result<(), AdapterError> {
        Ok(())
    }

    /// Send a message to the MockHSM
    fn send_message(&self, _uuid: Uuid, body: Vec<u8>) -> Result<Vec<u8>, AdapterError> {
        let command = CommandMessage::parse(body)
            .map_err(|e| err!(ConnectionFailed, "error parsing command: {}", e))?;

        let mut state = self
            .0
            .lock()
            .map_err(|e| err!(ConnectionFailed, "error obtaining state lock: {}", e))?;

        match command.command_type {
            CommandType::CreateSession => commands::create_session(&mut state, &command),
            CommandType::AuthSession => commands::authenticate_session(&mut state, &command),
            CommandType::SessionMessage => commands::session_message(&mut state, command),
            unsupported => fail!(ConnectionFailed, "unsupported command: {:?}", unsupported),
        }
    }
}
