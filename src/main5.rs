use futures::executor::block_on;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

async fn get_score() -> i32 {
    100
}

async fn get_user() -> String {
    thread::sleep(Duration::from_secs(2));
    format!("user score is: {}", get_score().await)
}

fn main() {
    let ret = block_on(get_user());
    println!("{}", ret);

    let server = TcpListener::bind("0.0.0.0:80").unwrap();
    loop {
        let (stream, _) = server.accept().unwrap();
        handler(stream);
    }
}

fn handler(mut stream: TcpStream) {
    let rsp = String::from("http/1.1 200 OK\r\n\r\n rust server");
    stream.write(rsp.as_bytes()).unwrap();
}
