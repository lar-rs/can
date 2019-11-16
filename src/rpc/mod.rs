// pub mod motor;
// pub mod analog;
pub mod digital;
// pub mod aouts;
use super::can::Data;
use jsonrpc_core::futures::future::{self, Future, FutureResult};
use jsonrpc_core::{Error, IoHandler, Result};
use jsonrpc_core_client::transports::local;
use jsonrpc_derive::rpc;

#[rpc]
pub trait Msg{
    /// Returns a protocol version
	// #[rpc(name = "protocolVersion")]
	// fn protocol_version(&self) -> Result<String>;
    /// read data msg
	#[rpc(name = "read")]
    fn read(&self,node : i32,index: i32,sub: u8) -> Result<Data>;
    /// write msg
    #[rpc(name = "write")]
    fn write(&self,node:i32,index: i32,sub: u8,value: Vec<u8>) -> Result<()>;
}