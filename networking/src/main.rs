use std::net::{TcpListener, TcpStream, Ipv4Addr, SocketAddr};
use std::io::{Read, Write, Result};

fn main(){
    // let ip = Ipv4Addr::new(127, 0, 0, 1);
    // let socket = SocketAddr::from((ip, 8080));

    // println!("Server address: {:?}", socket);

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);

            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client( mut stream: TcpStream) {
    // Implement client handling logic here
    let mut buffer = [0; 512];
    stream.read(&mut buffer).expect("Failed to read from stream");
    println!("Received: {}", String::from_utf8_lossy(&buffer));
}

