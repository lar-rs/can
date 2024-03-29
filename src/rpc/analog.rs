use jsonrpc_core::{Result};
use jsonrpc_derive::rpc;



#[rpc]
pub trait Analog {
    #[rpc(name = "analog_get_in01")]
    fn analog_get_in01(&self,node:u32) -> Result<u16>;
    #[rpc(name = "analog_get_in02")]
    fn analog_get_in02(&self,node:u32) -> Result<u16>;
    #[rpc(name = "analog_get_in03")]
    fn analog_get_in03(&self,node:u32) -> Result<u16>;
    #[rpc(name = "analog_get_in04")]
    fn analog_get_in04(&self,node:u32) -> Result<u16>;
    #[rpc(name = "analog_get_in05")]
    fn analog_get_in05(&self,node:u32) -> Result<u16>;
    #[rpc(name = "analog_get_out")]
    fn analog_get_out(&self,node:u32) -> Result<u16>;
    #[rpc(name = "analog_set_out")]
    fn analog_set_out(&self,node:u32,value: u16) -> Result<()>;
     #[rpc(name = "analog_get_temp01")]
    fn analog_get_temp01(&self,node:u32) -> Result<u16>;
     #[rpc(name = "analog_get_temp02")]
    fn analog_get_temp02(&self,node:u32) -> Result<u16>;
     #[rpc(name = "analog_get_temp03")]
    fn analog_get_temp03(&self,node:u32) -> Result<u16>;
    #[rpc(name = "analog_get_uart01")]
    fn analog_get_uart01(&self,node:u32) -> Result<Vec<u8>>;
    #[rpc(name = "analog_get_uart02")]
    fn analog_get_uart02(&self,node:u32) -> Result<Vec<u8>>;
    #[rpc(name = "analog_set_uart01")]
    fn analog_set_uart01(&self,node:u32,data:Vec<u8>) -> Result<()>;
    #[rpc(name = "analog_set_uart02")]
    fn analog_set_uart02(&self,node:u32,data:Vec<u8>) -> Result<()>;
}

