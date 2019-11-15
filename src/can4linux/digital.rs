use jsonrpc_core::{Result};

use serde_derive::{Deserialize,Serialize};
// use tide::{error::ResultExt, response, App, Context, EndpointResult};
use bitvec::prelude::*;
use super::can;
// use lazy_static::lazy_static;
// lazy_static! {
// }
use super::rpc::digital::Digital;

#[derive(Debug,Default, Clone, Serialize, Deserialize)]
pub struct DigitalNode;

use std::fmt::Write;



fn info(node : i32) -> String {
    let mut info = String::new();
    write!(&mut info, "digital:{}",node).unwrap(); // uses fmt::Write::write_fmt
    info
}
fn get_inputs(node:i32) -> u16 {
    unsafe{
        can::digital_get_input(node) as u16
    }
}
fn get_outputs(node:i32) -> u16 {
    unsafe{
        can::digital_get_output(node) as u16
    }
}
fn set_outputs(node:i32, value: u16) {
    unsafe{
        can::digital_set_output(node,value as u32);
    }
}


// use log::info;

///Digital1 #r`{"jsonrpc": "2.0", "method": "digital_info","params":[24],"id": 1}`
///Digital1 #r`{"jsonrpc": "2.0", "method": "digital_get_in00","params":[24],"id": 1}`
///Digital2 #r`{"jsonrpc": "2.0", "method": "digital_get_in00","params":[25],"id": 1}`
///
// pub trait DigitalNode {

// }






impl Digital for DigitalNode {

    fn get_info(&self,node:i32) -> Result<String> {
        Ok(info(node))
    }
    fn get_inputs(&self, node:i32) -> Result<u16> {
        Ok(get_inputs(node))
    }
    fn get_outputs(&self, node:i32) ->Result<u16> {
        Ok(get_outputs(node))
    }
    fn set_outputs(&self, node:i32,value:u16) -> Result<()> {
        Ok(set_outputs(node, value))
    }
    fn get_input00(&self, node:i32) ->Result<bool> {
        println!("Digital::get_input00");
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(0).unwrap())
    }
    fn get_input01(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(1).unwrap())
    }
    fn get_input02(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(2).unwrap())
    }
    fn get_input03(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(3).unwrap())
    }
    fn get_input04(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(4).unwrap())
    }
    fn get_input05(&self, node:i32) ->Result<bool> {
         let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(5).unwrap())
    }
    fn get_input06(&self, node:i32) ->Result<bool> {
         let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(6).unwrap())
    }
    fn get_input07(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(7).unwrap())
    }
    fn get_input08(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(8).unwrap())
    }
    fn get_input09(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(9).unwrap())
    }
    fn get_input10(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(10).unwrap())
    }
    fn get_input11(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(11).unwrap())
    }
    fn get_input12(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(12).unwrap())
    }
    fn get_input13(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(13).unwrap())
    }
    fn get_input14(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(14).unwrap())
    }
    fn get_input15(&self, node:i32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(15).unwrap())
    }
    fn get_output00(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(0).unwrap())
    }
    fn get_output01(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(1).unwrap())
    }
    fn get_output02(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(2).unwrap())
    }
    fn get_output03(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(3).unwrap())
    }
    fn get_output04(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(4).unwrap())
    }
    fn get_output05(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(5).unwrap())
    }
    fn get_output06(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(6).unwrap())
    }
    fn get_output07(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(7).unwrap())
    }
    fn get_output08(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(8).unwrap())
    }
    fn get_output09(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(9).unwrap())
    }
    fn get_output10(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(10).unwrap())
    }
    fn get_output11(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(11).unwrap())
    }
    fn get_output12(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(12).unwrap())
    }
    fn get_output13(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(13).unwrap())
    }
    fn get_output14(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(14).unwrap())
    }
    fn get_output15(&self, node:i32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(15).unwrap())
    }

    fn set_output00(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(0,value);
        self.set_outputs(node,dout)
    }
    fn set_output01(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(1,value);
        self.set_outputs(node,dout)
    }
    fn set_output02(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(2,value);
        self.set_outputs(node,dout)
    }
    fn set_output03(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(3,value);
        self.set_outputs(node,dout)
    }
    fn set_output04(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(4,value);
        self.set_outputs(node,dout)
    }
    fn set_output05(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(5,value);
        self.set_outputs(node,dout)
    }
    fn set_output06(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(6,value);
        self.set_outputs(node,dout)
    }
    fn set_output07(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(7,value);
        self.set_outputs(node,dout)
    }
    fn set_output08(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(8,value);
        self.set_outputs(node,dout)
    }
    fn set_output09(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(9,value);
        self.set_outputs(node,dout)
    }
    fn set_output10(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(10,value);
        self.set_outputs(node,dout)
    }
    fn set_output11(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(11,value);
        self.set_outputs(node,dout)
    }
    fn set_output12(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(12,value);
        self.set_outputs(node,dout)
    }
    fn set_output13(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(13,value);
        self.set_outputs(node,dout)
    }
    fn set_output14(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(14,value);
        self.set_outputs(node,dout)
    }
    fn set_output15(&self, node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(15,value);
        self.set_outputs(node,dout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn digital_node_test(node:i32) {
        unsafe{
            let din = get_inputs(node);
            println!("DIGITAL-IN:{:}",din);
            assert_eq!(din,0);
        }
    }
    #[test]
    fn digital_test() {
        digital_node_test(0x18);
        // digital_node_test(0x19);
        // digital_node_test(0x1a);
    }
}
