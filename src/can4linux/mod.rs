mod analog;
mod aouts;
mod motor;
mod digital;
pub mod bindings;


pub use self::bindings as can;
pub use self::analog::*;
pub use self::aouts::*;
pub use self::motor::*;
pub use self::digital::*;

pub use super::rpc;
use jsonrpc_core::{Result};
use super::rpc::Node;

fn read_value(node:i32,index:i32,sub: u8) -> Result<u32>{
    unsafe{
        Ok(can::read_unsigned(node,index,sub))
    }
}
fn write_value(node : i32,index: i32,sub: u8,len:u8,value: u32) -> Result<()>{
    unsafe{
        can::write_unsigned(node, index, sub,len,value);
    }
    Ok(())
}


fn read_long(node:i32,index:i32,sub:u8) -> Result<Vec<u8>> {
    // let mut frame:*mut can::Frame = can::Frame{
        // type_: 0x40,
        // buf: [0;256],
        // len: 0,
    // }
    // let mut err = 0;
    // unsafe{
        // err = can::canframe(node,index,sub,frame);
    // }
    Ok(Vec::new())
    // uart.from(analog_get_uart01(node));
}
///write long
fn write_long(node:i32,index:i32,sub:u8,data:Vec<u8>) -> Result<()> {
    // let mut frame = can::Frame{
        // type_: 0x21,
        // buf: data.as_mut_slice(),
        // len: 0,
    // }
    // let mut err = 0;
    // unsafe{
        // err = can::canframe(node,index,sub,frame.as_mut_ptr());
    // }
    Ok(())
    // uart.from(analog_get_uart01(node));
}



pub struct NodeObject;

impl Node for NodeObject {
    fn read_value(&self,node : i32,index: i32,sub: u8) -> Result<u32>{
        read_value(node,index,sub)
    }
    fn write_u32(&self,node:i32,index:i32,sub:u8,value:u32) -> Result<()> {
        write_value(node,index,sub,4,value)
    }
    fn write_u16(&self,node:i32,index:i32,sub:u8,value:u16) -> Result<()>{
        write_value(node,index,sub,2,value as u32)
    }
    fn write_u8(&self,node:i32,index:i32,sub:u8,value:u8) -> Result<()>{
        write_value(node,index,sub,1,value as u32)
    }
    fn write_bytes(&self,node:i32,index:i32,sub:u8,value:Vec<u8>) -> Result<()>{
        write_long(node,index,sub,value)
    }
    fn read_bytes(&self,node:i32,index:i32,sub:u8) -> Result<Vec<u8>> {
        read_long(node,index,sub)
    }
}

#[cfg(test)]
mod tests {
    #![feature(async_await)]
    use super::*;
    #[test]
    fn node_test() {

        // println!("ANALOG IN01:{:}",in01);
        // assert_eq!(in01,887);
    }
}

