pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub enum Message {
    /// Initiate a WebSocket session with the master node.
    Hello,
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
