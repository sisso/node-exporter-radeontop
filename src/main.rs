use std::error::Error;
use std::io::BufRead;

mod radeontop;
mod server;

fn main() -> Result<(), Box<dyn Error>> {
    server::Server::new(radeontop::RadeonListener::new()?, 9101)?.run()?;
    Ok(())
}
