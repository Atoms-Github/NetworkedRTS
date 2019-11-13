use std::net::SocketAddr;
use tokio::net::TcpStream;
use std::time::Duration;
use futures::future::{Future, IntoFuture};
use tokio::net::tcp::ConnectFuture;



//pub fn stall_thread_until_connection_success(target_ip : &String) -> TcpStream{ // This should totally return a Task<TCPStream> or a Future<TCPStream>
//    let addr = target_ip.to_string().parse::<SocketAddr>().unwrap();
//
//    let mut connection;
//    let mut connection_maybe;
//
//
//    loop {
//        connection = TcpStream::connect(&addr);
//        println!("Going to wait for connection.");
//        let meme_two = tokio::spawn(connection);
////        let meme = tokio::runtime::current_thread::block_on_all(connection);
////        let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
////        let s = runtime.block_on(connection);
//        println!("{:?}", meme_two);
//        println!("Wait over.");
//
//        panic!("Got here");
//        let connection_dcwct = TcpStream::connect(&addr);
//        connection_maybe = connection_dcwct.wait(); // Stall everything while waiting for connection.
//
//        match connection_maybe{
//            Ok(_) => {
//                break;
//            },
//            Err(_) => {
//                println!("Failed to connect to {} retrying...", &target_ip);
//                std::thread::sleep(Duration::from_millis(500));
//            },
//        }
//    }
//    let mut connection = connection_maybe.unwrap(); // Already checked valid earlier.
//    return connection;
//}
//
//
//
