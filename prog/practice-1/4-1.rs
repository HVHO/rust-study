use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main () -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:7893")?;

    for stream in listener.incoming() {

        println!("Connect Established!");

        let mut stream = stream?;
        let mut buf = [0; 512];

        stream.read(&mut buf)?;

        println!("data : {}",String::from_utf8_lossy(&buf));

    }

    Ok(())
}