/// `lscan` socketcan api binding to rust and jsonrpc.
/// 
///

pub mod error;
pub mod banner;
pub mod config;
pub mod message;
pub mod cli;
// pub mod can;
pub mod dbus;
// pub mod rpc;
pub mod cmd;
pub use error::CanError;


pub trait Datagramm  {
    fn tramsmit(&self,msg:message::Message) -> Result<message::Message,CanError>;
}