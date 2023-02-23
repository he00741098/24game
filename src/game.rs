pub mod game{
    use futures_util::stream::{SplitSink, SplitStream};
    use tokio::net::TcpStream;
    use tokio_tungstenite::WebSocketStream;
    use tungstenite::{WebSocket, Message};


pub struct Game{
//hold the game state
id:u32,
senders:Vec<SplitSink<WebSocketStream<TcpStream>, Message>>,
recievers:Vec<SplitStream<WebSocketStream<TcpStream>>>,



}

impl Game{




}




}