use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = match
        TcpListener::bind("127.0.0.1:25565") {
            Ok(l) => l,
            Err(e) => {
                println!("Could not listen on port");
                return;
            }
    };
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("Could not open Tcp stream");
                return;
            }
        };

        handle_connection(stream);    
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(_) => {},
        Err(e) => {
            println!("Could not read from stream");
            return;
        }
    }

    let get = b"GET / HTTP/1.1\r\n";

    let (file_name, status_line) = 
        if buf.starts_with(get) {
            ("index.html", "404 NOT FOUND")
        } else {
            ("404.html", "200 OK")
        };

    let page = match fs::read_to_string(file_name) {
        Ok(p) => p,
        Err(_) => return,
    };

    let response = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        page.len(),
        page
    );
    if let Err(e) = stream.write(response.as_bytes()) {
            println!("Could not write to the stream");
            return;
    }
    if let Err(e) = stream.flush() {
        println!("Could not flush stream");
        return;
    }
}