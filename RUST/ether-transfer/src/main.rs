use std::error::Error;

mod client;

fn main() -> Result<(), Box<dyn Error>> {
    client::run()?;
    Ok(())
}
