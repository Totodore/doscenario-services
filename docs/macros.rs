#[macro_export]
macro_rules! broadcast_message {
    ($sockets:expr, $message:expr) => {
        futures::future::join_all($sockets.map(|tx| tx.send($message)))
    };
}
