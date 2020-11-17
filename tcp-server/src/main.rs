use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // define 50byte buffer
    loop  { // read stream
        match stream.read(&mut data) {
            Ok(size) => {
                println!("stream size: {}", size);
                if size > 0 {
                    // echo
                    stream.write(&data[0..size]).unwrap(); // write back
                    break;
                } else {
                    println!("Read size 0, shutdown...");
                    break;
                }
            },
            Err(_) => {
                println!("An error occurred, terminating connection with{}", stream.peer_addr().unwrap()); // print peer address
                break;
            }
        }
    }
    println!("disconnected!")
}


fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();
    println!("Server is listening on port 8888");

    // accept connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // spawn a new thread
                thread::spawn(move || {
                    handle_client(stream);
                });
            }

            // connection failed
            Err(e) => {
                println!("Connection failed! Eror:{}", e);
            }
        }
    }
    drop(listener);
}
