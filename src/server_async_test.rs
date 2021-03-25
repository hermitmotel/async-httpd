use async_std::prelude::*;
use async_std::net::TcpStream;
use async_std::task;
use std::time::Instant;

const NO_REQUESTS: usize = 10000;

async fn async_tests() {
	let mut stream = TcpStream::connect("127.0.0.1:7878").await.unwrap();
    let request = b"GET / HTTP/1.1\r\n";

    stream.write(&request[0..]).await.unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    
    if !buffer.starts_with(b"HTTP/1.1 200 OK") {
        println!("receive invalid message");
    }
}

fn main() {
    let start = Instant::now();
    task::block_on(async {
        for _ in 0..NO_REQUESTS {
            async_tests().await;
        }
    });
    let end = Instant::now();

    println!("{:?}", end.duration_since(start));
}