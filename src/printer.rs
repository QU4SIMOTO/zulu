use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

pub struct Printer {
    addr: SocketAddr,
    timeout: Duration,
    stream: Option<TcpStream>,
}

impl Printer {
    pub fn new(addr: SocketAddr, timeout: u64) -> Self {
        Self {
            addr,
            timeout: Duration::from_secs(timeout),
            stream: None,
        }
    }

    pub fn write_bytes(self: &mut Self, buf: &[u8]) -> Result<(), std::io::Error> {
        let stream = self.get_stream()?;

        stream.write(buf)?;
        stream.flush()?;
        Ok(())
    }

    pub fn read_bytes(self: &mut Self) -> Result<Vec<u8>, std::io::Error> {
        let stream = self.get_stream()?;
        let mut res = Vec::new();
        let mut buf = [0; 1024];
        loop {
            match stream.read(&mut buf) {
                Ok(0) => break, // EOF
                Ok(n) if n > 1 => {
                    res.extend_from_slice(&buf[..n]);
                    if buf[n - 1] == b'"' {
                        break;
                    }
                }
                Ok(n) => res.extend_from_slice(&buf[..n]),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    dbg!("Read timeout reached.");
                    break;
                }
                Err(e) => return Err(e.into()),
            }
            stream.read(&mut buf)?;
        }
        return Ok(res);
    }

    fn get_stream(self: &mut Self) -> Result<&mut TcpStream, std::io::Error> {
        if self.stream.is_none() {
            let stream = TcpStream::connect(self.addr)?;
            stream.set_read_timeout(Some(self.timeout))?;
            stream.set_write_timeout(Some(self.timeout))?;
            self.stream = Some(stream);
        }
        Ok(self.stream.as_mut().unwrap())
    }
}
