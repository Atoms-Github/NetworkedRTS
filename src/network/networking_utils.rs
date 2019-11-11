use std::net::SocketAddr;
use tokio::net::TcpStream;
use std::time::Duration;
use futures::future::Future;

pub fn stall_thread_until_connection_success(target_ip : &String) -> TcpStream{
    let addr = target_ip.to_string().parse::<SocketAddr>().unwrap();

    let mut stream;
    let mut connection_maybe;


    loop {
        stream = TcpStream::connect(&addr);
        connection_maybe = stream.wait(); // Stall everything while waiting for connection.

        match connection_maybe{
            Ok(_) => {
                break;
            },
            Err(_) => {
                println!("Failed to connect to {} retrying...", &target_ip);
                std::thread::sleep(Duration::from_millis(500));
            },
        }
    }
    let mut connection = connection_maybe.unwrap(); // Already checked valid earlier.
    return connection;
}




