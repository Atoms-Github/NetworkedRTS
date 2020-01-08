


use std::sync::mpsc::{Receiver, Sender, channel};
use crate::network::networking_structs::PlayerID;
use crate::network::networking_message_types::{NetMessageType, start_inwards_codec_thread};
use std::net::{TcpListener, SocketAddr, TcpStream};
use std::thread;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;


pub struct NetworkingHub{
    output_messages_sender: Option<Sender<OwnedNetworkMessage>>,
    player_ids: Arc<Mutex<PlayerID>>,
    new_connections_registerer: Option<Sender<(PlayerID, TcpStream)>>
}

pub struct OwnedNetworkMessage{
    pub owner: PlayerID,
    pub message: NetMessageType
}

pub enum DistributableNetMessage{
    ToSingle(PlayerID, NetMessageType)
}



impl NetworkingHub{ // This isn't responsible for sending worlds. // TODO: Fix indentation.
pub fn new() -> NetworkingHub {
    NetworkingHub{
        output_messages_sender: None,
        player_ids: Arc::new(Mutex::new(0)),
        new_connections_registerer: None
    }
}
fn handle_new_socket(&self, stream: TcpStream){
    thread::spawn(move ||{

        let new_connections_sink = self.new_connections_registerer.unwrap().clone();
        let handle_id;
        {
            let mut next_player_id_lock = self.player_ids.lock().unwrap();
            handle_id = *next_player_id_lock;
            *next_player_id_lock = handle_id + 1;
        }
        new_connections_sink.send((handle_id, stream.try_clone().unwrap())).unwrap();


        let my_output_messages_sender = self.output_messages_sender.unwrap().clone();
        let receiver = start_inwards_codec_thread(stream); // This can reasonably easily be optimised to use one fewer thread per connection.
        loop{
            let message = receiver.recv().unwrap();
            let wrapped = OwnedNetworkMessage{
                owner: handle_id,
                message
            };
            my_output_messages_sender.send(wrapped).unwrap();
        }

    });
}
pub fn start_logic(mut self /* TODO: Ref might be enough. */, input_messages: Receiver<DistributableNetMessage>, addr: SocketAddr) -> Receiver<OwnedNetworkMessage>{

    // HandleIncomingConnections.
    let (out_sender, out_receiver) = channel();
    let (new_connections_sender, new_connections_receiver) = channel();

    self.new_connections_registerer = Some(new_connections_sender);
    self.output_messages_sender = Some(out_sender);

    thread::spawn( move ||{ // Listen for new connections.
        let socket = TcpListener::bind(&addr).expect("Unable to bind hosting address.");
        println!("Hosting on {}", hosting_ip);
        let id_counter = Arc::new(Mutex::new(0));
        for stream in socket.incoming() {
            self.handle_new_socket(stream.unwrap());
        }
    });




    let clients_map = Arc::new(Mutex::new(HashMap::new()));

    thread::spawn(||{ // Add new connections to dictionary.
        let my_receiver = new_connections_receiver;
        let my_clients_map = clients_map.clone();
        loop{
            let (id, stream) = my_receiver.recv().unwrap();
            let mut locked = my_clients_map.lock().unwrap();
            locked.insert(id, stream);
        }
    });

    thread::spawn(move ||{ // Distribute messages to connections.
        let mut my_clients_map = clients_map.clone();
        loop{
            let distributable_message = input_messages.recv().unwrap();
            let mut locked = clients_map.lock().unwrap();
            match distributable_message {
                DistributableNetMessage::ToSingle(target, msg) => {
                    msg.encode_and_send(locked.get_mut(&target).unwrap());
                }
            }
        }
    });

    return out_receiver;


}
}