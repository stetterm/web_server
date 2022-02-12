use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use web_server::pool;
use web_server::pool::ThreadPool;

fn main() {
    let listener = match
        TcpListener::bind("127.0.0.1:25565") {
            Ok(l) => l,
            Err(e) => {
                println!("Could not listen on port");
                return;
            }
    };
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("Could not open Tcp stream");
                return;
            }
        };

        pool.execute(|| {
            handle_connection(stream);  
        });  
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
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (file_name, status_line) = 
        if buf.starts_with(get) {
            ("index.html", "404 NOT FOUND")
        } else if buf.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("index.html", "HTTP/1.1 200 OK")
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