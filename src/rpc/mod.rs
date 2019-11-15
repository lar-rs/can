// pub mod motor;
// pub mod analog;
// pub mod digital;
// pub mod aouts;

use jsonrpc_derive::rpc;
use jsonrpc_core::{Result};

#[rpc]
pub trait  RpcCan{
    #[rpc(name = "read_value")]
    fn read_value(&mut self,node : i32,index: i32,sub: u8) -> Result<u32>;
    #[rpc(name = "write_u32")]
    fn write_u32(&mut self,node:i32,index: i32,sub: u8,value:u32) -> Result<()>;
    #[rpc(name = "write_u16")]
    fn write_u16(&mut self,node:i32,index: i32,sub: u8,value:u16) -> Result<()>;
    #[rpc(name = "write_u8")]
    fn write_u8(&mut self,node:i32,index: i32,sub: u8,value:u8) -> Result<()>;
    #[rpc(name = "write_bytes")]
    fn write_bytes(&mut self,node:i32,index: i32,sub: u8,value:Vec<u8>) -> Result<()>;
    #[rpc(name = "read_bytes")]
    fn read_bytes(&mut self,node:i32,index: i32,sub: u8) -> Result<Vec<u8>>;
}
