use std::error::Error;

pub trait Server {
    fn start(&self) -> Result<(), Box<dyn Error>>;
    fn stop(&self) -> Result<(), Box<dyn Error>>;
}
