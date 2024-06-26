//! A JSON-serialiable message format for WebSocket communication between master nodes and connected GUI clients.
//! 
//! This message frame is simpler than the one used for UWB, as the client messages are only transferred between master nodes and connected GUI clients. Using a client link not only provides
//! a way to control and monitor controller games, it's also useful to run diagnostics and do time-travel debugging, inspecting controller events and game states in real-time.


pub struct ClientFrame {

}

pub struct ClientHeader {
    /// A ten-byte long unique message identifier generated by the `nanoid` crate.
    pub id: String,
}

pub enum ClientPayload {
    /// Set global controller brightness as a percentage between 0.0 and 1.0.
    SetBrightness(f32),
}
