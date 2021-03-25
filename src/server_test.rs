use std::net::TcpStream;
use std::io::*;
use std::time::Instant;

const NO_REQUESTS: usize = 10000;

fn std_tests() {
	let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let request = b"GET / HTTP/1.1\r\n";

    stream.write(&request[0..]).unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    if !buffer.starts_with(b"HTTP/1.1 200 OK") {
        println!("receive invalid message");
    }
}

fn main() {
    let start = Instant::now();
    for _ in 0..NO_REQUESTS {
        std_tests();
    }
    let end = Instant::now();

    println!("{:?}", end.duration_since(start));
}