use std::env;
use std::fs;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    const CONTENT_ENCODING: [&str; 6] = ["gzip", "br", "compress", "exi", "pack200-gzip", "zstd"];

    let args: Vec<String> = env::args().collect();
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                // println!("accepted new connection");
                let mut buffer = [0; 1024];
                let _ = _stream.read(&mut buffer);
                let message = String::from_utf8_lossy(&buffer[..]);
                let mut write_user_agent_info = false;
                // println!("{}", message);
                let lines: Vec<&str> = message.split("\r\n").collect();
                for line in 0..lines.len() {
                    println!("line");
                    let header: Vec<&str> = lines[line].split(" ").collect();
                    for a in header.clone() {
                        println!("header: {}", a);
                    }
                    println!("TEST: {}", &header[0]);
                    match header[0] {
                        "GET" => {
                            // println!("info {}", info[0]);
                            let info: Vec<&str> = header[1].split("/").collect();
                            match info[1] {
                                "" => {
                                    _stream
                                        .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                                        .expect("200");
                                }
                                "echo" => {
                                    let mut response;
                                    let encode_info: Vec<&str> =
                                        lines[2].replace(":", "").split(" ").collect();
                                    if encode_info[0] == "Accept-Encoding" {
                                        response = format!("HTTP/1.1 200 OK\r\nContent-Encoding: {}\r\nContent-Type: text/plain\r\nContent-length: {}\r\n\r\n{}", encode_info, encode_info ,info[2].len(), info[2]);
                                    } else {
                                        response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-length: {}\r\n\r\n{}", info[2].len(), info[2]);
                                    }
                                    _stream.write(response.as_bytes()).expect("200");
                                }
                                "user-agent" => {
                                    write_user_agent_info = true;
                                }
                                "files" => {
                                    let file =
                                        format!("{}{}", args[2].clone(), &info[2..].join("/"));
                                    // println!("file: {}", file);
                                    match fs::read_to_string(file) {
                                        Ok(file_content) => {
                                            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-length: {}\r\n\r\n{}", file_content.len(), file_content);
                                            _stream.write(response.as_bytes()).expect("200");
                                        }
                                        Err(e) => {
                                            print!("Err: {}", e);
                                            _stream
                                                .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                                                .expect("404");
                                        }
                                    }
                                }
                                _ => {
                                    _stream
                                        .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                                        .expect("404");
                                }
                            }
                        }
                        "POST" => {
                            let info: Vec<&str> = header[1].split("/").collect();
                            match info[1] {
                                "files" => {
                                    let file =
                                        format!("{}{}", args[2].clone(), &info[2..].join("/"));
                                    match fs::write(
                                        file,
                                        &lines[lines.len() - 1].replace("\x00", ""),
                                    ) {
                                        Ok(()) => {
                                            let response = format!("HTTP/1.1 201 Created\r\n\r\n");
                                            _stream.write(response.as_bytes()).expect("201");
                                        }
                                        Err(e) => {
                                            println!("Err: {}", e);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        "User-Agent:" => {
                            println!("print user agent: {}", header[1]);
                            if write_user_agent_info {
                                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", header[1].len(), &header[1]);
                                // println!("response: {}", response);
                                _stream.write(response.as_bytes()).expect("200");
                                write_user_agent_info = false;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
