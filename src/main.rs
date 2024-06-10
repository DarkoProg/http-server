use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 1024];
                let _ = _stream.read(&mut buffer);
                let message = String::from_utf8_lossy(&buffer[..]);
                println!("{}", message);
                for line in message.split("\r\n") {
                    let header: Vec<&str> = line.split(" ").collect();
                    if header[0] == "GET" {
                        if header[1] == "/" {
                            _stream
                                .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                                .expect("200");
                        } else {
                            _stream
                                .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                                .expect("404");
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
