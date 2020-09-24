use crate::http::Request; //crate è la root
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    // associated func (::)
    pub fn new(addr: String) -> Self {
        // special type as alias of the struct
        Self {
            addr, // like JS if key is the same ov values omit :
        }
    }

    // method, similar to associated func but we have to pass self
    pub fn run(self) {
        // this func has ownership of self
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; //byte slice come GO []byte
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            print!("Ricevo: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                // è come dire &buffer as &[u8]
                                Ok(request) => {
                                    println!("{:?}", request);
                                }
                                Err(e) => println!("Failed to load request {}", e),
                            }
                        }
                        Err(err) => println!("{}", err),
                    }
                }
                Err(err) => {
                    println!("error: {}", err);
                }
            }
        }
    }
}
