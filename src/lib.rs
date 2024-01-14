use serde::{Serialize, Deserialize};
use chrono::DateTime;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Packet {
    pub timestamp: DateTime<chrono::Utc>,
    pub message: Message,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
/// The possible interactions with client devices or other game controllers.
pub enum Message {
    Request(Request),
    Response(Response),
    Notice(Notice),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Response {
    /// Sent by the master node to a client or controller to indicate that 
    /// the connection was successful.
    Connected {
        id: u16,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Request {
    /// Initiate a WebSocket session with the master node.
    Hello {
        /// Request to join the mesh as a GUI client.
        is_client: bool,
    },
    /// Set the percentual brightness of the controller LEDs between 0.0 and 1.0.
    SetBrightness(f32),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Notice {
    /// Terminate the socket session.
    Farewell,
    /// Synchronize controller time.
    Tick(u16),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
