use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum UwbPacketError {
    /// An error occurred while serializing or deserializing the packet.
    SerializeError,
    /// The magic string "LEDswarm" was not found at the start of the byte buffer, so the received datagram is not a valid LEDswarm UWB packet.
    NoMagicString(String),
}

/// A data packet sent between controllers in the UWB mesh.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UwbPacket {
    //pub uuid: String,
    pub sender_id: u16,
    pub target_id: Option<u16>,
    pub timestamp: String,
    pub ranging_bytes: [u8; 4],
    pub message:   UwbMessage,
    /// How many times this message will be redirected by the mesh before being dropped.
    pub lifetime: u8,
}

impl From<UwbPacket> for Vec<u8> {
    fn from(packet: UwbPacket) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice("LEDswarm".as_bytes());
        buffer.append(&mut bincode::serialize(&packet).unwrap());
        buffer.append(&mut packet.ranging_bytes.to_vec());

        buffer
    }
}

impl TryFrom<Vec<u8>> for UwbPacket {
    type Error = UwbPacketError;

    fn try_from(vec: Vec<u8>) -> Result<Self, UwbPacketError> {
        if core::str::from_utf8(&vec[0 .. 8]).unwrap() == "LEDswarm" {
            match bincode::deserialize::<Self>(&vec["LEDswarm".len() .. vec.len() - 4]) {
                Ok(mut packet) => {
                    // Extract the last four bytes, which are ranging data, and put them into the message, so they can be restored if the message is serialized again.
                    packet.ranging_bytes = [vec[vec.len() - 4], vec[vec.len() - 3], vec[vec.len() - 2], vec[vec.len() - 1]];
                    Ok(packet)
                },
                Err(_e) => Err(UwbPacketError::SerializeError),
            }
        } else {
            Err(UwbPacketError::NoMagicString(String::from_utf8(vec[0 .. 7].to_vec()).unwrap()))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GameMode {
    /// The controller is currently not in a game session.
    Idle,
    /// Keep your own light green while pushing (softly) on the other light rods to make them red. Being
    /// green or red determines if you're still in the round or not. The last remaining player wins.
    LastOneStanding,
    /// All players are divided into two or more groups and each group is assigned a color. To take over territory, get
    /// your own controller within range of another, try to push it and it may take on the color of yours. The game is
    /// finished and a winner may be declared when all controllers have the same color.
    Territory,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum UwbMessage {
    Acknowledged {
        /// The UUID of the message being acknowledged.
        uuid: Uuid,
    },
    /// Try to request a controller ID from the master node to join a nearby UWB mesh.
    JoinRequest,
    /// Sent by the master node after accepting a `JoinRequest` to inform a client that it has been assigned an ID in the mesh.
    Welcome {
        /// The recipient's newly assigned ID.
        controller_id: u16,
    },
    /// Notify the mesh that this controller is about to leave the session.
    Bye,
    /// Set global brightness of the controller LEDs as a percentage between 0.0 and 1.0.
    SetBrightness(f32),
    /// Initiate a new game round with the specified game mode.
    StartRound(GameMode),
    /// Notify the mesh that the current game round has ended.
    EndRound,
    /// Synchronize controller time using an incrementing, repeating 16-bit counter fired every 10 microseconds.
    Tick(u16),
    /// Responsitivity check broadcast every 10 milliseconds to check if the mesh is still alive.
    Ping,
    /// Responsitivity check response to a `Ping` message.
    Pong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize_join_request() {
        let packet = UwbPacket {
            sender_id: 0,
            target_id: None,
            timestamp: "now".to_string(),
            ranging_bytes: [0, 0, 0, 0],
            message:   UwbMessage::JoinRequest,
            lifetime: 1,
        };

        let encoded = Vec::from(packet.clone());
        assert_eq!(packet, UwbPacket::try_from(encoded).unwrap());
    }

    #[test]
    fn serialize_deserialize_welcome() {
        let packet = UwbPacket {
            sender_id: 0,
            target_id: None,
            timestamp: "now".to_string(),
            ranging_bytes: [0, 0, 0, 0],
            message:   UwbMessage::Welcome {
                controller_id: 1,
            },
            lifetime: 1,
        };

        let encoded = Vec::from(packet.clone());
        assert_eq!(packet, UwbPacket::try_from(encoded).unwrap());
    }
}