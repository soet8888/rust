use super::worker;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn start(port: &str) {
    let listener = TcpListener::bind(format!("{}{}", "127.0.0.1:", port)).unwrap();
    let pool = worker::ThreadPool::new(4);
    println!("Sever listening on localhost:{}....", port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down...");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get_hb = b"GET /hb HTTP/1.1";
    let get = b"GET / HTTP/1.1\r\n";
    // --snip--
    let (status_line, filename) = if buffer.starts_with(get) || buffer.starts_with(get_hb) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
