//! A data structure for the header of a frame, which contains metadata about the frame, such as the sender and target IDs, the universe number, and the current tick.

use serde_derive::{Serialize, Deserialize};
use nanoid::nanoid;
use chrono::DateTime;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FrameHeader {
    /// When the frame was sent, as a string representation of a `DateTime` object.
    pub timestamp: String,
    /// How many times this message will be redirected by the mesh before being dropped.
    pub lifetime: u8,
    /// A small unique message identifier generated by the `nanoid` crate.
    pub message_id: String,
    /// The assigned ID of the sender of the frame. If the sender does not have an ID yet (trying to join the mesh), this field is set to 65535.
    pub sender_id: u16,
    /// The assigned ID of the target of the frame, broadcasting to all nodes if set to `None`.
    pub target_id: Option<u16>,
    // If set, the sender of the frame expects an acknowledgement from the receiver and will retry until it receives one.
    pub requires_acknowledgement: bool,
    /// The current repeating synchronization tick as an integer between 0 and 65535.
    pub current_tick: u16,
    /// Used to separate different logical networks in the same physical network, to play multiple games next to each other without interference.
    pub universe: u8,
    /// The ranging data from the UWB module, which is used to calculate the distance between the sender and the receiver.
    pub ranging_bytes: [u8; 4],
}

/*
self.uwb_out_tx.send(UwbPacket {
            sender_id: u16::MAX,
            target_id: Some(0),
            timestamp: "now".to_string(),
            ranging_bytes: [0, 0, 0, 0],
            message:   UwbMessage::JoinRequest,
            lifetime: 1,
        }).unwrap(); */

impl FrameHeader {
    pub fn new() -> Self {
        Self {
            timestamp: DateTime::<chrono::Local>::from(chrono::Local::now()).to_rfc3339(),
            lifetime: 2,
            message_id: nanoid!(10),
            sender_id: u16::MAX,
            requires_acknowledgement: false,
            target_id: None,
            current_tick: 0,
            universe: 0,
            ranging_bytes: [0; 4],
        }
    }
}