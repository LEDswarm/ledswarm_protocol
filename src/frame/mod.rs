//! A data structure representing a single data frame of the LEDswarm protocol.
//!
//! More specifically, these are protocol frames used to transmit data over the UWB network. They consist of a frame header and a few different types of payloads,
//! which can be either high-level game commands or low-level control messages. The frame header contains metadata about the frame, such as the sender and target IDs,
//! the universe number, the current small-scale time and whether the messages requires `ACK` confirmation or not.
//! 
//! A simple join request can be composed like this using the builder pattern:
//! 
//! ```rust
//! use ledswarm_protocol::frame::{Frame, ControllerMessage};
//! 
//! let join_request = Frame::new()
//!   // Tell the master node that we would like to join the mesh network.
//!   .message(ControllerMessage::JoinRequest)
//!   // Seek delivery notification for this UWB frame and retry sending
//!   // the message until acknowledged by the master node.
//!   .require_confirmation()
//!   // Target master nodes only. Every master node has an ID of zero.
//!   .target_id(0)
//!   // Add the current time tick to the frame.
//!   .tick(tick);
//! ```

use serde_derive::{Serialize, Deserialize};

pub mod error;
pub mod header;
pub mod payload;

pub use self::header::FrameHeader;
pub use self::payload::{
    FramePayload,
    ClientMessage,
    ControllerMessage,
    ProtocolMessage,
    InternalMessage,
};
pub use self::error::FrameError;

// A container for a single frame of data from the UWB mesh.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Frame {
    /// Metadata about the frame, such as the sender and target IDs, the universe number, and the current tick.
    pub header:  FrameHeader,
    /// The actual payload of the frame, which can be a game-level command or an internal network command.
    pub payload: FramePayload,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            header: FrameHeader::new(),
            payload: FramePayload::Empty,
        }
    }

    pub fn join_request(tick: u16) -> Self {
        Self::new()
            .message(ControllerMessage::JoinRequest)
            .require_confirmation()
            .target_id(0)
            .tick(tick)
    }

    /// Set a game-level command as the message payload of the frame.
    pub fn message(mut self, msg: ControllerMessage) -> Self {
        self.payload = FramePayload::ControllerMessage(msg);
        self
    }

    /// Set an internal network command as the message payload of the frame.
    pub fn protocol_message(mut self, protocol_msg: ProtocolMessage) -> Self {
        self.payload = FramePayload::ProtocolMessage(protocol_msg);
        self
    }

    pub fn internal_message(mut self, msg: InternalMessage) -> Self {
        self.payload = FramePayload::InternalMessage(msg);
        self
    }

    pub fn client_message(mut self, msg: ClientMessage) -> Self {
        self.payload = FramePayload::ClientMessage(msg);
        self
    }

    pub fn lifetime(mut self, lifetime: u8) -> Self {
        self.header.lifetime = lifetime;
        self
    }

    pub fn sender_id(mut self, id: u16) -> Self {
        self.header.sender_id = id;
        self
    }

    pub fn target_id(mut self, id: u16) -> Self {
        self.header.target_id = Some(id);
        self
    }

    pub fn tick(mut self, current_tick: u16) -> Self {
        self.header.current_tick = current_tick;
        self
    }

    pub fn universe(mut self, num: u8) -> Self {
        self.header.universe = num;
        self
    }

    pub fn require_confirmation(mut self) -> Self {
        self.header.requires_acknowledgement = true;
        self
    }
}

impl From<Frame> for Vec<u8> {
    fn from(packet: Frame) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice("LEDswarm".as_bytes());
        buffer.append(&mut bincode::serialize(&packet).unwrap());
        buffer.append(&mut packet.header.ranging_bytes.to_vec());

        buffer
    }
}

impl TryFrom<Vec<u8>> for Frame {
    type Error = FrameError;

    fn try_from(vec: Vec<u8>) -> Result<Self, FrameError> {
        if core::str::from_utf8(&vec[0 .. 8]).unwrap() == "LEDswarm" {
            match bincode::deserialize::<Self>(&vec["LEDswarm".len() .. vec.len() - 4]) {
                Ok(mut packet) => {
                    // Extract the last four bytes, which are ranging data, and put them into the message, so they can be restored if the message is serialized again.
                    packet.header.ranging_bytes = [vec[vec.len() - 4], vec[vec.len() - 3], vec[vec.len() - 2], vec[vec.len() - 1]];
                    Ok(packet)
                },
                Err(_e) => Err(FrameError::SerializeError),
            }
        } else {
            Err(FrameError::NoMagicString(String::from_utf8(vec[0 .. 7].to_vec()).unwrap()))
        }
    }
}
