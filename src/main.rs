use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener};
use std::thread;

const TEMPLATE_FOLDER: &'static str = "templates";

const HTML_RESPONSE_HEADER: &'static [u8] =
    b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
const CSS_RESPONSE_HEADER: &'static [u8] =
    b"HTTP/1.1 200 OK\r\nContent-Type: text/css; charset=UTF-8\r\n\r\n";
const JS_RESPONSE_HEADER: &'static [u8] =
    b"HTTP/1.1 200 OK\r\nContent-Type: application/javascript; charset=UTF-8\r\n\r\n";

fn read_static_file(file_path: &str) -> Vec<u8> {
    println!("Reading file: {}", file_path);
    let mut file = std::fs::File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn read_template_file(file_path: &str) -> Vec<u8> {
    let file_path_ = format!("{}/{}", TEMPLATE_FOLDER, file_path);
    let mut file = std::fs::File::open(file_path_).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let url = request.split_whitespace().nth(1).unwrap();

    let (file_path, response_header) = if url.starts_with("/static/") {
        (url.trim_start_matches('/'), CSS_RESPONSE_HEADER)
    } else if url.ends_with(".js") {
        (url.trim_start_matches('/'), JS_RESPONSE_HEADER)
    }else {
        let file_path = "index.html";
        (file_path, HTML_RESPONSE_HEADER)
    };

    let response_body = if url.starts_with("/static/") {
        read_static_file(file_path)
    } else {
        read_template_file(file_path)
    };

    stream.write(response_header).unwrap();
    stream.write(&response_body).unwrap();
    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on port 8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_client(stream);
        });
    }
}





// use std::net::{TcpStream, TcpListener};
// use std::io::{Read, Write};
// use std::thread;

// fn handle_read(mut stream: &TcpStream) {
//     let mut buffer = [0; 1024];
//     match stream.read(&mut buffer) {
//         Ok(_num_bytes_read) => {
//             let req_str = String::from_utf8_lossy(&buffer);
//             println!("Received request: {}", req_str);
//         }
//         Err(e) => println!("Failed reading request: {}", e),
//         }
// }

// fn handle_write(mut stream:TcpStream){
//     let response = b"http/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<!DOCTYPE html><html><head><title>Sigma Male</title><style>body { background-color: #111 }h1 { font-size:4cm; text-align: center; color: black;text-shadow: 0 0 2mm red}</style></head><body><h1>Dimon pisyunyarik</h1></body></html>\r\n";
//     match stream.write(response) {
//         Ok(_) => println!("Response sent!"),
//         Err(e) => println!("Failed sending response: {}!", e),
//     }
// }
// fn handle_client(stream:TcpStream) {
//     handle_read(&stream);
//     handle_write(stream);
// }

// fn main() {
//     let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
//     println!("Server listening on port 8080");

//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 thread::spawn(|| {
//                     handle_client(stream);
//                 });
//             }
//             Err(e) => println!("Failed connecting to client: {}", e),
//         }
//     }
// }

// use std::net::{TcpListener, TcpStream};
// use std::io::{Read, Write};
// use std::thread;

// fn handle_client(mut stream: TcpStream) {
//     let mut buffer = [0; 512];
//     stream.read(&mut buffer).unwrap();
//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

//     let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
//     stream.write(response).unwrap();
//     stream.flush().unwrap();
// }

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
//     println!("Listening on port 8080");

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//         thread::spawn(|| {
//             handle_client(stream);
//         });
//     }
// }