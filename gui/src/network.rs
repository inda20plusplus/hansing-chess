use std::io::Read;
use std::io::{Error, ErrorKind};
use std::net::{TcpListener, TcpStream};

struct Listener {
    hist_packets: Vec<u8>,
    listener: TcpListener,
    connection: Result<TcpStream, Error>,
}

impl Listener {
    pub fn new(port: u16) -> Listener {
        let addr = format!("127.0.0.1:{}", port);
        Listener {
            hist_packets: Vec::new(),
            listener: TcpListener::bind(addr).unwrap(),
            connection: Err(Error::new(
                ErrorKind::NotConnected,
                "No connection established",
            )),
        }
    }

    pub fn check_connected(&mut self) -> Result<TcpStream, Error> {
        match self.listener.accept() {
            Ok((stream, _addr)) => Ok(stream),
            Err(e) => Err(e),
        }
    }

    pub fn next(&mut self) -> Result<u8, Error> {
        let mut ret: [u8; 1] = [0];
        match self.connection.as_ref() {
            Ok(mut stream) => stream.read(&mut ret),
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, "No data to read")),
        };
        Ok(ret[0])
    }
}
