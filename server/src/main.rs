use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener 
        = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();   // shadowing
        println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; 
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents_index = fs::read_to_string("index.html").unwrap();
    
        let response =  format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents_index.len(),
            contents_index
        );
        
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    } else {
        let status_line_404 = "HTTP/1.1 404 NOT FOUND";
        let contents_404 = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line_404,
            contents_404.len(),
            contents_404
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    
}