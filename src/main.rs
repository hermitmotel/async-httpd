use async_std::prelude::*;
use async_std::fs;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::task::{self, block_on, spawn};
use futures::stream::StreamExt;

use std::time::Duration;

async fn handle_connection(mut stream: TcpStream) {
	// Read the first 1024 bytes of data from the stream
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).await.unwrap();

	let get = b"GET / HTTP/1.1\r\n";
	let sleep = b"GET /sleep HTTP/1.1\r\n";

	// Respond with greetings or a 404,
	// depending on the data in the request
	let (status_line, filename) = if buffer.starts_with(get) {
		("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
	} else if buffer.starts_with(sleep) {
		task::sleep(Duration::from_secs(5)).await;
		("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
	} else {
		("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
	};
	let contents = fs::read_to_string(filename).await.unwrap();

	// Write response back to the stream,
	// and flush the stream to ensure the response is sent back to the client
	let response = format!("{}{}", status_line, contents);
	stream.write(response.as_bytes()).await.unwrap();
	stream.flush().await.unwrap();
}

async fn http_listen() {
	// Listen for incoming TCP connections on localhost port 7878
	let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
	let mut incoming = listener.incoming();

	while let Some(tcpstream) = incoming.next().await {
		handle_connection(tcpstream.unwrap()).await;
		//spawn(handle_connection(tcpstream.unwrap()));
	}
}

fn main() {
	block_on(http_listen())
}