use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ControllerMessage {
    JoinRequest,
    JoinResponse {
        assigned_id: u16,
    },
}