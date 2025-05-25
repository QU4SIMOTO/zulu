use crate::{sdg::SdgCommand, upload::UploadLocation};
use std::{
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    path::Path,
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

    pub fn run_sdg(self: &mut Self, command: SdgCommand) -> Result<(), Box<dyn std::error::Error>> {
        self.write_bytes(&command.to_string().into_bytes())?;
        let response = self.read_bytes()?;
        if response.len() > 0 {
            println!("{}", String::from_utf8_lossy(&response));
        }
        Ok(())
    }

    pub fn upload_file<P: AsRef<Path>>(
        self: &mut Self,
        loc: UploadLocation,
        file: P,
        dest: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = fs::read(file.as_ref())?;
        let size = data.len();
        let data_format = "B"; // todo support other formats:  B (raw binary format), C (AR compressed), and P (hexadecimal format PNG data)
        let dest_ext = dest.split_once(".").unwrap_or(("", "")).1;
        let row_bytes = ""; // only applies to images
        let header = format!("~DY{loc}:{dest},{data_format},{dest_ext},{size},{row_bytes},");

        let mut buffer = header.into_bytes();
        buffer.extend_from_slice(&data);
        self.write_bytes(&buffer)
    }

    fn get_stream(self: &mut Self) -> Result<&mut TcpStream, Box<dyn std::error::Error>> {
        if self.stream.is_none() {
            let stream = TcpStream::connect(self.addr)?;
            stream.set_read_timeout(Some(self.timeout))?;
            self.stream = Some(stream);
        }
        Ok(self.stream.as_mut().unwrap())
    }

    fn write_bytes(self: &mut Self, bs: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let stream = self.get_stream()?;

        stream.write(bs)?;
        stream.flush()?;
        Ok(())
    }

    fn read_bytes(self: &mut Self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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
                    println!("Read timeout reached.");
                    break;
                }
                Err(e) => return Err(e.into()),
            }
            stream.read(&mut buf)?;
        }
        return Ok(res);
    }
}
