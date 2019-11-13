use tokio::io::WriteHalf;

use crate::network::networking_structs::*;
use tokio::net::TcpStream;
use tokio_io::_tokio_codec::FramedWrite;
use crate::network::*;

pub struct ClientHandle {
//    pub write_channel: FramedWrite<WriteHalf<TcpStream>, dans_codec::Bytes>,
    pub write_channel: WriteHalf<TcpStream>,
    pub message_box: MessageBox
}
