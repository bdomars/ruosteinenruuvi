mod message;
pub use message::*;

mod btle;
pub use btle::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;