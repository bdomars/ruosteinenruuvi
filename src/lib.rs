mod message;
pub use message::*;

mod btle;
pub use btle::*;

mod config;
pub use config::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = anyhow::Result<T, Error>;
