#![warn(clippy::pedantic)]
use std::io::{stderr, Read, Write};
use std::net::TcpStream;
use std::{error, result};

type Error = Box<dyn error::Error + Send + Sync>;
type Result<T> = result::Result<T, Error>;

fn main() -> Result<()> {
    let header = "GET / HTTP/1.0\r\nHost: fake\r\n\r\n";
    let mut success = false;

    for host in ["127.0.0.1:8000", "[::1]:8000", "localhost:8000"].map(ToOwned::to_owned) {
        let mut stream = TcpStream::connect(&host)?;
        stream.write_all(header.as_bytes())?;

        let mut buf = [0; 128];
        if stream.read(&mut buf).is_ok() {
            success = true;
            writeln!(
                stderr(),
                "{} worked: {}\n",
                host,
                std::str::from_utf8(&buf)?
            )?;
        } else {
            writeln!(stderr(), "{host} failed\n")?;
        }
    }
    if !success {
        panic!("oh no!")
    };
    Ok(())
}
