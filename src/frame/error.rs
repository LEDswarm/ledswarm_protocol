//! Error types for when something goes wrong with an UWB frame, like serialization or deserialization errors.

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum FrameError {
    /// An error occurred while serializing or deserializing the packet.
    SerializeError,
    /// The magic string "LEDswarm" was not found at the start of the byte buffer, so the received datagram is not a valid LEDswarm UWB packet.
    NoMagicString(String),
}