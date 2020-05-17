//pub struct MessageBox {
//    pub items: Arc<Mutex<Vec<ExternalMsg>>>,
//}
//
//impl MessageBox {
//    //    pub fn spawn_thread_message_box_fill(&self, connection_readable: TcpStream){
////        let message_box_mutex = Arc::clone(&self.items); // However this works :)
////
////        thread::spawn(move ||{
////            let inc_messages = start_inwards_codec_thread(connection_readable);
////
////            loop{
////                let message = inc_messages.recv().unwrap();
////
////                {
////                    let mut mutex_lock= Mutex::lock(&message_box_mutex).unwrap();
////                    println!("Adding to message box: {:?}", e);
////                    mutex_lock.push(message);
////                }
////
////            }
////        });
////    }
//    pub fn spawn_thread_fill_from_receiver(&self, receiver: Receiver<ExternalMsg>){
//        let meme = receiver;
//        let message_box_mutex = Arc::clone(&self.items); // However this works :)
//
//        thread::spawn(move ||{
//            let dream = meme;
//            loop{
//                let item = dream.recv();
//                match item{
//                    Ok(net_message) => {
//                        {
//                            let mut mutex_lock= Mutex::lock(&message_box_mutex).unwrap();
//                            mutex_lock.push(net_message);
//                            std::mem::drop(mutex_lock); // Just to doubley ensure lock is dropped.
//                        }
//                    },
//                    Err(err) => {
//                        panic!("Error initing filling message box from reciever. {}", err);
//                    },
//                }
//            }
//        });
//    }
//    pub fn new() -> MessageBox {
//        MessageBox {
//            items: Arc::new(Mutex::new(vec![]))
//        }
//    }
//    pub fn spawn_thread_read_cmd_input(&self){
//        let (sender, reciever) = channel::<ExternalMsg>();
//        thread::spawn(||{
//            let sink = sender;
//            let stdin = io::stdin();
//            for line in stdin.lock().lines() {
//                sink.send(ExternalMsg::LocalCommand(LocalCommandInfo{
//                    command: line.expect("Problem reading std:io input line.")
//                })).unwrap();
//            }
//        });
//        self.spawn_thread_fill_from_receiver(reciever);
//    }
//}