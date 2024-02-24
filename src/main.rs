use std::error::Error;
use std::io::BufRead;

mod radeontop;

fn main() -> Result<(), Box<dyn Error>> {
    let mut listener = radeontop::RadeonListener::new()?;
    loop {
        let data = listener.next();
        println!("{:?}", data);
    }
    Ok(())
}
