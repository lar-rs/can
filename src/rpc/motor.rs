use jsonrpc_core::{Result};
use jsonrpc_derive::rpc;

#[rpc]
pub trait DoppelMotor {
    #[rpc(name = "get_info")]
    fn get_info(&self,node:i32)-> Result<String>;
    #[rpc(name = "get_uart11")]
    fn get_uart11(&self,node:i32)-> Result<Vec<u8>>;
    #[rpc(name = "get_uart11")]
    fn get_uart12(&self,node:i32)-> Result<Vec<u8>>;
    #[rpc(name = "set_uart11")]
    fn set_uart11(&self,node:i32,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "set_uart12")]
    fn set_uart12(&self,node:i32,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "set_baut11")]
    fn set_baut11(&self,node:i32,bautrate:u32)-> Result<()>;
    #[rpc(name = "set_baut12")]
    fn set_baut12(&self,node:i32,bautrate:u32)-> Result<()>;
    #[rpc(name = "get_uart21")]
    fn get_uart21(&self,node:i32)-> Result<Vec<u8>>;
    #[rpc(name = "get_uart21")]
    fn get_uart22(&self,node:i32)-> Result<Vec<u8>>;
    #[rpc(name = "set_uart21")]
    fn set_uart21(&self,node:i32,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "set_uart22")]
    fn set_uart22(&self,node:i32,data:Vec<u8>)-> Result<()>;
    #[rpc(name = "set_baut21")]
    fn set_baut21(&self,node:i32,bautrate:u32)-> Result<()>;
    #[rpc(name = "set_baut22")]
    fn set_baut22(&self,node:i32,bautrate:u32)-> Result<()>;
    #[rpc(name = "set_stepper1")]
    fn set_stepper1(&self,node:i32) -> Result<()>;
    #[rpc(name = "set_stepper2")]
    fn set_stepper2(&self,node:i32) -> Result<()>;
    #[rpc(name = "get_endschalter1")]
    fn get_endschalter1(&self,node:i32) -> Result<bool>;
    #[rpc(name = "get_endschalter2")]
    fn get_endschalter2(&self,node:i32) -> Result<bool>;
    #[rpc(name = "invert_endschalter1")]
    fn invert_endschalter1(&self,node:i32) -> Result<()>;
    #[rpc(name = "invert_endschalter2")]
    fn invert_endschalter2(&self,node:i32) -> Result<()>;
    #[rpc(name = "set_command1")]
    fn set_command1(&self,node:i32,cmd:u16) -> Result<()>;
    #[rpc(name = "set_command2")]
    fn set_command2(&self,node:i32,cmd:u16) -> Result<()>;
    #[rpc(name = "set_position1")]
    fn set_position1(&self,node:i32,pos:u16) -> Result<()>;
    #[rpc(name = "set_position2")]
    fn set_position2(&self,node:i32,pos:u16) -> Result<()>;
    #[rpc(name = "get_position1")]
    fn get_position1(&self,node:i32) -> Result<u16>;
    #[rpc(name = "get_position2")]
    fn get_position2(&self,node:i32) -> Result<u16>;
    #[rpc(name = "set_max_position1")]
    fn set_max_position1(&self,node:i32,max:u32) -> Result<()>;
    #[rpc(name = "set_max_position2")]
    fn set_max_position2(&self,node:i32,max:u32) -> Result<()>;
    #[rpc(name = "set_velocity1")]
    fn set_velocity1(&self,node:i32,velocity:u32) -> Result<()>;
    #[rpc(name = "set_velocity2")]
    fn set_velocity2(&self,node:i32,velocity:u32) -> Result<()>;
    #[rpc(name = "set_stromsollwert1")]
    fn set_stromsollwert1(&self,node:i32,value:u32) -> Result<()>;
    #[rpc(name = "set_stromsollwert2")]
    fn set_stromsollwert2(&self,node:i32,value:u32) -> Result<()>;
    #[rpc(name = "set_stirrer1")]
    fn set_stirrer1(&self,node:i32) -> Result<()>;
    #[rpc(name = "set_stirrer2")]
    fn set_stirrer2(&self,node:i32) -> Result<()>;
    #[rpc(name = "start_stirrer1")]
    fn start_stirrer1(&self,node:i32) -> Result<()>;
    #[rpc(name = "start_stirrer2")]
    fn start_stirrer2(&self,node:i32) -> Result<()>;
    #[rpc(name = "stop_stirrer1")]
    fn stop_stirrer1(&self,node:i32) -> Result<()>;
    #[rpc(name = "stop_stirrer2")]
    fn stop_stirrer2(&self,node:i32) -> Result<()>;
    #[rpc(name = "set_delay1")]
    fn set_delay1(&self,node:i32,delay:u16) -> Result<()>;
    #[rpc(name = "set_delay2")]
    fn set_delay2(&self,node:i32,delay:u16) -> Result<()>;
}
