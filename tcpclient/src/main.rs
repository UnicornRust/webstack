use std::{io::{Read, Write}, net::TcpStream};

fn main() {


    let mut server  = TcpStream::connect("127.0.0.1:3000").unwrap();
    Write::write(&mut server, "hello".as_bytes()).unwrap();

    let mut buffer = [0; 10];
    Read::read(&mut server, &mut buffer).unwrap();
    println!("receiver server message: {}", String::from_utf8_lossy(&buffer));
}
