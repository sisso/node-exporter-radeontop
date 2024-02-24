use std::{error, thread};
use std::sync::{Arc, Mutex};

use crate::radeontop::{RadeonData, RadeonListener};

pub fn run_server(mut listener: RadeonListener, port: i32) -> Result<(), Box<dyn error::Error>> {
    let state = Arc::new(Mutex::new(Option::<RadeonData>::None));

    // trigger listener in a different thread so data don't get stale
    {
        let thread_state = state.clone();
        thread::spawn(move || loop {
            let data = listener.next().unwrap();
            let mut state = thread_state.lock().unwrap();
            *state = Some(data);
        });
    }

    // instantiate server
    let server = match tiny_http::Server::http(format!("0.0.0.0:{}", port)) {
        Ok(value) => value,
        Err(err) => {
            return Err(format!("Failed to start server: {}", err).into());
        }
    };

    // handle requests
    loop {
        let req = server.recv()?;
        let data = state.lock().unwrap();
        req.respond(tiny_http::Response::from_string(format!("{:?}", data)))?;
    }
}
