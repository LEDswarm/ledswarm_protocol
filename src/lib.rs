//! A message protocol to enable communication between LEDswarm nodes.
//! 
//! This protocol implements controller-controller communication over UWB as well as WebSocket interactions over HTTP to connected GUI clients. Two separate encodings
//! are used for the two different communication channels. The UWB protocol uses a binary encoding, while the WebSocket protocol uses JSON.

pub mod client;
pub mod packet;
pub mod frame;

pub use self::packet::{UwbPacket, UwbMessage, GameMode};
pub use self::frame::{
    Frame,
    FrameError,
    FrameHeader,
    FramePayload,
    ClientMessage,
    ControllerMessage,
    InternalMessage,
    ProtocolMessage,
};
/* 
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum InternalMessage {
    Packet(UwbPacket),
    Frame(Frame),
    /// The current average change of acceleration (jolt) experienced by the controller enclosure, as a vector sum.
    AccelerometerDelta(f32),
    /// A raw accelerometer reading.
    AccelerometerRaw { x: f32, y: f32, z: f32 },
}
*/