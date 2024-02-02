/// Something that happened during execution.
/// Meant for debugging by a human.
#[derive(Debug, Clone)]
pub struct Event {
    /// What happened in the event.
    pub text: String,
    /// How important the event was.
    pub severity: Severity,
}

/// How important the event is.
#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Info,
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
