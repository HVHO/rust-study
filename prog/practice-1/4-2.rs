use std::io::prelude::*;
use std::net::TcpStream;

fn main () -> std::io::Result<()>{

    let mut stream = TcpStream::connect("localhost:7893")?;

    let buf = "Hello, world".as_bytes();

    stream.write(buf)?;

    Ok(())
}