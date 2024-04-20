//! Definitions for different types of frame payloads to distinguish between high-level and low-level transactions.

use serde_derive::{Serialize, Deserialize};

mod controller;
mod protocol;

pub use self::controller::ControllerMessage;
pub use self::protocol::ProtocolMessage;

use crate::frame::Frame;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum FramePayload {
    /// High-level transactions like joining the mesh, sending game commands or other messages.
    ControllerMessage(ControllerMessage),
    /// Low-level transactions like ACKs and other control messages.
    ProtocolMessage(ProtocolMessage),
    /// A message that is sent to or received from a connected GUI client.
    ClientMessage(ClientMessage),
    /// A message meant for internal use only, not to be sent over the network.
    // InternalMessage(InternalMessage),
    /// Empty payload, mostly used for control messages which don't carry any data.
    Empty,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ClientMessage {
    SetBrightness(f32),
    /// Start a game with a specific name.
    StartRound(String),
    /// End the game round if one is currently active.
    EndRound,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum InternalMessage {
    /// The current average change of acceleration (jolt) experienced by the controller enclosure, as a vector sum.
    AccelerometerJoltDelta(f32),
    /// A raw accelerometer reading.
    AccelerometerRaw { x: f32, y: f32, z: f32 },
    ClientMessage(ClientMessage),
    Frame(Box<Frame>),
}
