use crate::rpc::Msg;
use crate::rpc::analog::Analog;
use crate::rpc::aouts::AOuts;
// use crate::rpc::motor as motor1;
// use crate::rpc::motor as motor2;
use super::*;
use jsonrpc_core::{Result};


impl Msg for Can {
    fn read_data(&self,node:u32,index:u16,sub:u8) -> Result<Data> {
        let data = self.read(Addr::new(node,index,sub))?;
        Ok(data)
    }
    fn write_data(&self,node:u32,index:u16,sub:u8,data: Data) -> Result<()>{
		self.write(Addr::new(node,index,sub),data)?;
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
    fn analog_set_out(&self,node:u32,value: u16) -> Result<()> {
        self.write(Addr::new(node,0x6111,1),value.into())?;
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
        let data = self.read(Addr::new(0x1c,0x6411,0))?;
        Ok(data.into())
    }
    fn get_outs(&self, num: u8) -> Result<u16> {
        let data = self.read(Addr::new(0x1c,0x6411,num))?;
        Ok(data.into())
    }
    fn set_outs(&self, num: u8, val: u16) -> Result<()>{
        self.write(Addr::new(0x1c,0x6411,num),val.into())?;
        Ok(())
    }
}
