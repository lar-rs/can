use crate::rpc::Msg;
use crate::rpc::analog::Analog;
use crate::rpc::aouts::AOuts;
// use crate::rpc::motor as motor1;
// use crate::rpc::motor as motor2;
use super::*;
use jsonrpc_core::{IoHandler, Result};


impl Msg for Can {
    fn read(&self,node : i32,index: i32,sub: u8) -> Result<Data> {
        Ok(Data{
            bytes: Vec::new(),
            len: 0x40,
        })
    }
    fn write(&self,node:i32,index: i32,sub: u8,data: Data) -> Result<()> {
		// self.write(addr: Addr, tx: Data)
		Ok(())
    }
}

impl Analog for Can {
    fn analog_get_in01(&self) -> Result<u16> {
        Ok(0)
    }
    fn analog_get_in02(&self) -> Result<u16> {
        Ok(0)
    }
    fn analog_get_in03(&self) -> Result<u16> {
        Ok(0)
    }
    fn analog_get_in04(&self) -> Result<u16> {
        Ok(0)
    }
    fn analog_get_in05(&self) -> Result<u16> {
        Ok(0)
    }
    fn analog_get_out(&self) -> Result<u16> {
        Ok(0)
    }
    fn analog_set_out(&self,value: u16) -> Result<()> {
        Ok(())
    }
    fn analog_get_temp01(&self) -> Result<u16> {
        Ok(0)
    }
    fn analog_get_temp02(&self) -> Result<u16> {
        Ok(0)
    }
    fn analog_get_temp03(&self) -> Result<u16> {
        Ok(0)
    }
    fn analog_get_uart01(&self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
    fn analog_get_uart02(&self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
    fn analog_set_uart01(&self,data:Vec<u8>) -> Result<()> {
        Ok(())
    }
    fn analog_set_uart02(&self,data:Vec<u8>) -> Result<()> {
        Ok(())
    }
}

impl AOuts for Can{
    fn outs_count(&self) -> Result<u8> {
        Ok(0)
    }
    fn get_outs(&self, num: u8) -> Result<u16> {
        Ok(0)
    }
    fn set_outs(&self, num: u8, val: u16) -> Result<()>{
        Ok(())
    }
}
