use std::error;

use crate::radeontop::RadeonListener;

pub struct Server {
    pub listener: RadeonListener,
    pub port: i32,
}

impl Server {
    pub fn new(listener: RadeonListener, port: i32) -> Result<Server, Box<dyn error::Error>> {
        Ok(Self { listener, port })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn error::Error>> {
        let server = match tiny_http::Server::http(format!("0.0.0.0:{}", self.port)) {
            Ok(value) => value,
            Err(err) => {
                return Err(format!("Failed to start server: {}", err).into());
            }
        };

        loop {
            let req = server.recv()?;
            let data = self.listener.next()?;
            req.respond(tiny_http::Response::from_string(format!("{:?}", data)))?;
        }
    }
}
