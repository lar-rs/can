/// `lscan` socketcan api binding to rust and jsonrpc.
/// 
///

pub mod error;
pub mod banner;
pub mod config;
pub mod message;
pub mod cli;
// pub mod can;
pub mod setup;
pub mod ondbus;
pub mod pipe;
// pub mod rpc;
pub use error::CanError;

pub use self::ondbus as io;

pub trait Datagramm  {
    fn tramsmit(&self,msg:message::Message) -> Result<message::Message,CanError>;
}