use std::error::Error;
use std::io::BufRead;

mod radeontop;
mod server;

fn main() -> Result<(), Box<dyn Error>> {
    server::run_server(radeontop::RadeonListener::new()?, 9101)?;
    Ok(())
}
