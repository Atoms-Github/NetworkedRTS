

use crate::network::networking_structs::*;
use std::net::TcpStream;

pub struct ClientHandle {
//    pub write_channel: FramedWrite<WriteHalf<TcpStream>, dans_codec::Bytes>,
    pub write_channel: TcpStream,
    pub message_box: MessageBox,
//    pub properties: PlayerProperties // TODO2: This shouldn't be in here. We should keep all the game state together.
}
pub struct ServerReceptionData{
    pub new_player_handles: Vec<(PlayerID, ClientHandle)>,
    pub next_player_id: PlayerID
}
impl ServerReceptionData{
    pub fn new() -> ServerReceptionData{
        ServerReceptionData{
            new_player_handles: vec![],
            next_player_id: 4
        }
    }
}