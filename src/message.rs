use signal_hook;

use std::sync;
use std::sync::atomic;

/// Handle receiving and setting messages
pub struct MessageHandler {
    message: sync::Arc<atomic::AtomicUsize>,
}

/// Message signaling the application to draw/update the popup
///
/// This message is only set by the application
pub const DRAW: usize = 0;

/// Message signaling the application to hide the popup
///
/// This message is received from the user
pub const HIDE: usize = 1;

/// Message signaling the application to do nothing\
/// 
/// This message is only set by the application
pub const NONE: usize = 2;

/// Message signaling the application to show the popup
///
/// This message is received from the user
pub const SHOW: usize = 3;

impl MessageHandler {
    /// Load the message value
    pub fn message(&self) -> usize {
        self.message.load(atomic::Ordering::SeqCst)
    }

    /// Set message value to `message`
    pub fn set_message(&self, message: usize) {
        self.message.store(message, atomic::Ordering::SeqCst);
    }

    /// Create and setup a new `MessageHandler`
    ///
    /// # Errors
    /// - if setting up signal handlers fail
    pub fn setup() -> Result<Self, String> {
        let message = sync::Arc::new(atomic::AtomicUsize::new(DRAW));

        // SIGUSR1 (signal 10) sets the message to `HIDE`
        // SIGUSR2 (signal 12) sets the message to `SHOW`
        signal_hook::flag::register_usize(
            signal_hook::consts::SIGUSR1,
            message.clone(),
            HIDE,
        )
        .map_err(|error| error.to_string())?;
        signal_hook::flag::register_usize(
            signal_hook::consts::SIGUSR2,
            message.clone(),
            SHOW,
        )
        .map_err(|error| error.to_string())?;

        Ok(Self { message })
    }
}
