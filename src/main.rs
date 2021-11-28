use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let ip = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip).unwrap();
    let linebreak = "-".repeat(50);
    println!("\n\n{}", linebreak);
    println!("Rust web server running on: {}", ip);

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Connection established 😎✌️");
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    
    if buffer.starts_with(get) {
        rq_response(&stream, "return.html", "HTTP/1.1 200 OK");
    }
    else {
        rq_response(&stream, "404.html", "HTTP/1.1 404 NOT FOUND");
    }
}

fn rq_response(mut stream: &TcpStream, html_file: &str, html_response: &str){
        let html_contents = fs::read_to_string(html_file).unwrap();
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}", html_response,
            html_contents.len(),
            html_contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();    

}