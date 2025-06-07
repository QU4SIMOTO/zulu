use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use tracing::{debug, trace};

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
        trace!("Writing bytes to socket");
        debug!("data: {buf:?}");
        stream.write(buf)?;
        stream.flush()?;
        Ok(())
    }

    pub fn read_bytes(self: &mut Self) -> Result<Vec<u8>, std::io::Error> {
        let stream = self.get_stream()?;
        let mut res = Vec::new();
        let mut buf = [0; 1024];
        loop {
            trace!("reading from socket");
            match stream.read(&mut buf) {
                Ok(0) => {
                    trace!("read 0 bytes, finishing read EOF");
                    break; // EOF
                }
                Ok(n) if n > 1 => {
                    trace!("read {n} bytes");
                    debug!("data: {:?}", &buf[..n]);
                    res.extend_from_slice(&buf[..n]);
                    // NOTE: may need to be smarter about this.
                    if buf[n - 1] == b'"' {
                        trace!("read '\"' delim, assumed end of response");
                        break;
                    }
                }
                Ok(n) => {
                    trace!("read 1 byte");
                    debug!("data: {:?}", &buf[..n]);
                    res.extend_from_slice(&buf[..n])
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    trace!("Read timeout reached.");
                    break;
                }
                Err(e) => return Err(e.into()),
            }
        }
        return Ok(res);
    }

    fn get_stream(self: &mut Self) -> Result<&mut TcpStream, std::io::Error> {
        if self.stream.is_none() {
            trace!("connecting to address: {}", self.addr);
            let stream = TcpStream::connect(self.addr)?;
            trace!("setting read timeout: {:?}", self.timeout);
            stream.set_read_timeout(Some(self.timeout))?;
            trace!("setting write timeout: {:?}", self.timeout);
            stream.set_write_timeout(Some(self.timeout))?;
            self.stream = Some(stream);
        }
        Ok(self.stream.as_mut().unwrap())
    }
}
