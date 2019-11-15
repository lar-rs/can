use jsonrpc_core::{Result};
use jsonrpc_derive::rpc;

#[rpc]
pub trait Motor {
    #[rpc(name = "get_info")]
    fn get_info(&self,node:i32)-> Result<String>;
    #[rpc(name = "get_uart01")]
    fn get_uart01(&self,node:i32)-> Result<Vec<u8>>;
    #[rpc(name = "get_uart01")]
    fn get_uart02(&self,node:i32)-> Result<Vec<u8>>;
    #[rpc(name = "set_uart01")]
    fn set_uart01(&self,node:i32,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "set_uart02")]
    fn set_uart02(&self,node:i32,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "set_baut01")]
    fn set_baut01(&self,node:i32,bautrate:u32)-> Result<()>;
    #[rpc(name = "set_baut02")]
    fn set_baut02(&self,node:i32,bautrate:u32)-> Result<()>;
    #[rpc(name = "set_stepper")]
    fn set_stepper(&self,node:i32) -> Result<()>;
    #[rpc(name = "get_endschalter")]
    fn get_endschalter(&self,node:i32) -> Result<bool>;
    #[rpc(name = "invert_endschalter")]
    fn invert_endschalter(&self,node:i32) -> Result<()>;
    #[rpc(name = "set_command")]
    fn set_command(&self,node:i32,cmd:u16) -> Result<()>;
    #[rpc(name = "set_command")]
    fn set_position(&self,node:i32,pos:u16) -> Result<()>;
    #[rpc(name = "get_position")]
    fn get_position(&self,node:i32) -> Result<u16>;
    #[rpc(name = "set_max_position")]
    fn set_max_position(&self,node:i32,max:u32) -> Result<()>;
    #[rpc(name = "set_max_position")]
    fn set_velocity(&self,node:i32,velocity:u32) -> Result<()>;
    #[rpc(name = "set_stromsollwert")]
    fn set_stromsollwert(&self,node:i32,value:u32) -> Result<()>;
    #[rpc(name = "set_stirrer")]
    fn set_stirrer(&self,node:i32) -> Result<()>;
    #[rpc(name = "start_stirrer")]
    fn start_stirrer(&self,node:i32) -> Result<()>;
    #[rpc(name = "stop_stirrer")]
    fn stop_stirrer(&self,node:i32) -> Result<()>;
    #[rpc(name = "stop_stirrer")]
    fn set_delay(&self,node:i32,delay:u16) -> Result<()>;
}
