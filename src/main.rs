#![warn(clippy::pedantic)]
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{error, result};

type Error = Box<dyn error::Error + Send + Sync>;
type Result<T> = result::Result<T, Error>;

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8000")?;

    let header = "GET / HTTP/1.0\r\nHost: fake\r\n\r\n";
    stream.write_all(header.as_bytes())?;

    let mut buf = [0; 128];
    let _ = stream.read(&mut buf)?;
    println!("{}", std::str::from_utf8(&buf)?);
    Ok(())
}
