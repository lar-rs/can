use jsonrpc_core::{Result};
use jsonrpc_derive::rpc;

#[rpc]
pub trait DoppelMotor {
    #[rpc(name = "get_info")]
    fn get_info(&self,node:u32)-> Result<String>;
    #[rpc(name = "get_uart01")]
    fn get_uart01(&self,node:u32)-> Result<Vec<u8>>;
    #[rpc(name = "get_uart01")]
    fn get_uart02(&self,node:u32)-> Result<Vec<u8>>;
    #[rpc(name = "set_uart01")]
    fn set_uart01(&self,node:u32,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "set_uart02")]
    fn set_uart02(&self,node:u32,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "set_baut01")]
    fn set_baut01(&self,node:u32,bautrate:u32)-> Result<()>;
    #[rpc(name = "set_baut02")]
    fn set_baut02(&self,node:u32,bautrate:u32)-> Result<()>;
    #[rpc(name = "set_option1")]
    fn set_option1(&self,node:u32,value:u8) -> Result<()>;
    #[rpc(name = "set_option2")]
    fn set_option2(&self,node:u32,value:u8) -> Result<()>;
    #[rpc(name = "get_endschalter1")]
    fn get_endschalter1(&self,node:u32) -> Result<u8>;
    #[rpc(name = "get_endschalter2")]
    fn get_endschalter2(&self,node:u32) -> Result<u8>;
    #[rpc(name = "invert_endschalter1")]
    fn invert_endschalter1(&self,node:u32,value:u8) -> Result<()>;
    #[rpc(name = "invert_endschalter2")]
    fn invert_endschalter2(&self,node:u32,value:u8) -> Result<()>;
    #[rpc(name = "set_command1")]
    fn set_command1(&self,node:u32,cmd:u8) -> Result<()>;
    #[rpc(name = "set_command2")]
    fn set_command2(&self,node:u32,cmd:u8) -> Result<()>;
    #[rpc(name = "goto_position1")]
    fn goto_position1(&self,node:u32,pos:u16) -> Result<()>;
    #[rpc(name = "goto_position2")]
    fn goto_position2(&self,node:u32,pos:u16) -> Result<()>;
    #[rpc(name = "get_position1")]
    fn get_position1(&self,node:u32) -> Result<u16>;
    #[rpc(name = "get_position2")]
    fn get_position2(&self,node:u32) -> Result<u16>;
    #[rpc(name = "set_max_position1")]
    fn set_max_position1(&self,node:u32,max:u16) -> Result<()>;
    #[rpc(name = "set_max_position2")]
    fn set_max_position2(&self,node:u32,max:u16) -> Result<()>;
    #[rpc(name = "set_velocity1")]
    fn set_velocity1(&self,node:u32,velocity:u8) -> Result<()>;
    #[rpc(name = "set_velocity2")]
    fn set_velocity2(&self,node:u32,velocity:u8) -> Result<()>;
    #[rpc(name = "set_stromsollwert1")]
    fn set_stromsollwert1(&self,node:u32,value:u16) -> Result<()>;
    #[rpc(name = "set_stromsollwert2")]
    fn set_stromsollwert2(&self,node:u32,value:u16) -> Result<()>;
    #[rpc(name = "set_stirrer1")]
    fn set_stirrer1(&self,node:u32,value: u8) -> Result<()>;
    #[rpc(name = "set_stirrer2")]
    fn set_stirrer2(&self,node:u32,value: u8) -> Result<()>;
    #[rpc(name = "set_delay1")]
    fn set_delay1(&self,node:u32,delay:u8) -> Result<()>;
    #[rpc(name = "set_delay2")]
    fn set_delay2(&self,node:u32,delay:u8) -> Result<()>;
}
