use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use std::env;
use std::process::Command;

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            //return String::from_utf8_lossy(&buf);
            println!("Request from {}!",stream.peer_addr().unwrap());
            },
        Err(e) => println!("Stream read error: {}",e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let args: Vec<String> = env::args().collect();
    let action = &args[2];
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{res:\"ok\"}\r\n";
    match stream.write(response) {
        Ok(_) => {} //println!("Response to {} sent!",stream.peer_addr().unwrap()),
        Err(e) => println!("Failed sending response: {}",e),
    }
    match Command::new("/bin/sh").args(&["-c",action]).spawn() {
        Ok(_) => {} //println!("Action executed!"),
        Err(e) => println!("Action error: {}",e)
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    let listener = TcpListener::bind(format!("{}:{}","0.0.0.0",port)).unwrap();
    println!("Listening on 0.0.0.0:{}",port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}",e);
            }
        }
    }
}