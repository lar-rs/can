pub mod motor;
pub mod analog;
pub mod digital;
pub mod aouts;

use super::can::Data;
use jsonrpc_core::futures::future::{Future};
use jsonrpc_core::{Result};
// use jsonrpc_core_client::transports::local;
use jsonrpc_derive::rpc;

#[rpc]
pub trait Msg{
    /// Returns a protocol version
	// #[rpc(name = "protocolVersion")]
	// fn protocol_version(&self) -> Result<String>;
    /// read data msg
	#[rpc(name = "read_data")]
    fn read_data(&self,node:u32,index:u16,sub:u8) -> Result<Data>;
    /// write msg
    #[rpc(name = "write_data")]
    fn write_data(&self,node:u32,index:u16,sub:u8,data: Data) -> Result<()>;
}