use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ProtocolMessage {
    Acknowledged {
        /// The unique identifier of the message being acknowledged.
        message_id: String,
    },
    Tick(u16),
}