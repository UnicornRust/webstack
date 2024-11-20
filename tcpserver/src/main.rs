use std::{ io::{Read, Write}, net::TcpListener};

fn main() {
    let server  = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("server started at 127.0.0.1:3000");

    // 获取一次连接，可以使用 acept()
    // incoming() 可以获取到一个迭代器, 监听到 tcpStream 上的连接
    for stream in server.incoming() {
        // stream 获取到的是原始的二进制字节流
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        Read::read(&mut stream, &mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer);
        println!("Request: {}", request);
        Write::write(&mut stream, request.as_bytes()).unwrap();
    }
}
