


use std::sync::mpsc::{Receiver, Sender, channel};
use crate::network::networking_structs::PlayerID;
use crate::network::networking_message_types::{NetMessageType, start_inwards_codec_thread};
use std::net::{TcpListener, SocketAddr, TcpStream};
use std::thread;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;


pub struct NetworkingHub{
    
}

pub struct OwnedNetworkMessage{
    pub owner: PlayerID,
    pub message: NetMessageType
}

pub enum DistributableNetMessage{
    ToSingle(PlayerID, NetMessageType)
}

fn handle_new_socket(stream: TcpStream, messages_stream: Sender<OwnedNetworkMessage>, player_ids: Arc<Mutex<PlayerID>>, connections_registry: Sender<(PlayerID, TcpStream)>){
    thread::spawn(move ||{
        let my_id;
        {
            let mut next_player_id_lock = player_ids.lock().unwrap();
            my_id = *next_player_id_lock;
            *next_player_id_lock = my_id + 1;
        }
        connections_registry.send((my_id, stream.try_clone().unwrap())).unwrap();
        let receiver = start_inwards_codec_thread(stream); // This can reasonably easily be optimised to use one fewer thread per connection.
        loop{
            let message = receiver.recv().unwrap();
            let wrapped = OwnedNetworkMessage{
                owner: my_id,
                message
            };
            messages_stream.send(wrapped).unwrap();
        }

    });
}

impl NetworkingHub{ // This isn't responsible for sending worlds. // TODO: Fix indentation.
pub fn start_logic(self, input_messages: Receiver<DistributableNetMessage>, addr: SocketAddr) -> Receiver<OwnedNetworkMessage>{

    // HandleIncomingConnections.
    let (out_sender, out_receiver) = channel();
    let (new_connections_sender, new_connections_receiver) = channel();
    thread::spawn( move ||{ // Listen for new connections.
        let socket = TcpListener::bind(&addr).expect("Unable to bind hosting address.");
        let id_counter = Arc::new(Mutex::new(0));
        for stream in socket.incoming() {
            handle_new_socket(stream.unwrap(), out_sender.clone(),
                              id_counter.clone(), new_connections_sender.clone());
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