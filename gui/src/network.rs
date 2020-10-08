use std::io::{Error, ErrorKind};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct Networker {
    listener: Option<TcpListener>,
    connection: Result<TcpStream, Error>,
}

impl Networker {
    pub fn new(port: u16) -> Networker {
        let addr = format!("127.0.0.1:{}", port);
        Networker {
            listener: Some(TcpListener::bind(addr).unwrap()),
            connection: Err(Error::new(
                ErrorKind::NotConnected,
                "No connection established",
            )),
        }
    }

    pub fn connect(ip_addr: (u8, u8, u8, u8), port: u16) -> Networker {
        let addr = format!(
            "{}.{}.{}.{}:{}",
            ip_addr.0, ip_addr.1, ip_addr.2, ip_addr.3, port
        );
        Networker {
            listener: None,
            connection: Ok(TcpStream::connect(addr).unwrap()),
        }
    }

    pub fn check_connected(&mut self) -> Result<TcpStream, Error> {
        if self.listener.is_none() {
            panic!("Listener is None but check for connection occurs");
        }
        match self.listener.as_ref().unwrap().accept() {
            Ok((stream, _addr)) => Ok(stream),
            Err(e) => Err(e),
        }
    }

    pub fn read(&mut self) -> Result<u8, Error> {
        let mut ret: [u8; 1] = [0];
        match self.connection.as_ref() {
            Ok(mut stream) => stream.read(&mut ret),
            Err(_e) => return Err(Error::new(ErrorKind::InvalidData, "No data to read")),
        };
        Ok(ret[0])
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, Error> {
        match self.connection.as_ref() {
            Ok(mut stream) => stream.write(data),
            Err(_e) => return Err(Error::new(ErrorKind::InvalidData, "Invalid data to send")),
        };
        Ok(data.len())
    }
}
