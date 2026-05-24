//! Server-side request dispatch hook (business logic in `openpfe-server`).

use crate::envelope::Envelope;

/// Handles one validated envelope and returns the response envelope.
pub trait RequestHandler: Send + Sync {
    fn handle(&self, envelope: Envelope) -> Envelope;
}
