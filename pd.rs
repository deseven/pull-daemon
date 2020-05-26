use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use std::env;
use std::process::{Command,Stdio};

fn handle_read(mut stream: &TcpStream) -> String {
    let mut buf = [0u8 ;4096];
    let mut request = String::from("");
    match stream.read(&mut buf) {
        Ok(_) => {
            println!("Request from {}!",stream.peer_addr().unwrap());
            request = String::from_utf8_lossy(&buf).to_lowercase();
        },
        Err(e) => println!("Stream read error: {}",e),
    }
    return request;
}

fn handle_write(mut stream: TcpStream,response: &[u8]) {
    //let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{res:\"ok\"}\r\n";
    match stream.write(response) {
        Ok(_) => {} //println!("Response to {} sent!",stream.peer_addr().unwrap()),
        Err(e) => println!("Failed sending response: {}",e),
    }
}

fn handle_client(stream: TcpStream) {
    let args: Vec<String> = env::args().collect();
    let action = &args[2];
    let request = handle_read(&stream);
    if request.starts_with("get /pull") == true || request.starts_with("post /pull") == true {
        println!("Executing `{}`",action);
        handle_write(stream,b"HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{res:\"ok\"}\r\n");
        let proc = Command::new("/bin/sh").args(&["-c",action]).stdin(Stdio::null()).spawn().expect("failed to run command");
        let output = proc.wait_with_output().expect("failed to wait on command");
        assert!(output.status.success());
    } else {
        handle_write(stream,b"HTTP/1.1 403 Forbidden\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{res:\"failed\"}\r\n");
    }
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