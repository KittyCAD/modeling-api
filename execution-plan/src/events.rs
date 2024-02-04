//! Events can be logged during execution.
//! Used in the visual debugger.

use crate::Address;

/// Something that happened during execution.
/// Meant for debugging by a human.
#[derive(Debug, Clone)]
pub struct Event {
    /// What happened in the event.
    pub text: String,
    /// How important the event was.
    pub severity: Severity,
    /// This event might be about a particular address.
    /// Debuggers might want to visualize this.
    pub related_address: Option<Address>,
}

impl Event {
    /// New event, with other fields set to their default.
    pub fn new(text: String, severity: Severity) -> Self {
        Self {
            text,
            severity,
            related_address: None,
        }
    }
}

/// How important the event is.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Severity {
    /// Error
    Error,
    /// Info
    Info,
    /// Debug
    Debug,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Severity::Error => "Error",
            Severity::Info => "Info",
            Severity::Debug => "Debug",
        };
        write!(f, "{s}")
    }
}

/// Tracks events that occur during execution.
#[derive(Debug, Default, Clone)]
pub struct EventWriter {
    inner: Vec<Event>,
}

impl EventWriter {
    /// Add a new event.
    pub fn push(&mut self, event: Event) {
        self.inner.push(event);
    }
    /// Iterate over all events.
    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.inner.iter()
    }
    /// Remove all events, resetting the writer.
    pub fn drain(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.inner)
    }
}
