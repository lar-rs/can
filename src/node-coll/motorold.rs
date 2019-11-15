use jsonrpc_core::futures::future::{Future};
use jsonrpc_core::{Result};
// use jsonrpc_core_client::transports::local;
use jsonrpc_derive::rpc;

use bitvec::prelude::*;

use super::analog;
use super::digital;
use super::doppelmotor;


/// #r`{"jsonrpc": "2.0", "method": "protocolVersion"}`
/// #r`{"jsonrpc": "2.0", "method": "analog_get_in01"}`











impl CanRpc for CanMio {
    fn protocol_version(&self) -> Result<String> {
		Ok("rpc_version1".to_owned())
	}

    fn digital_get_din(&self,node:i32) ->Result<u16>{
        Ok(digital::get_input(node))
    }
    fn digital_get_dout(&self,node:i32) ->Result<u16> {
        Ok(digital::get_output(node))
    }
    fn digital_set_dout(&self,node:i32,value:u16) -> Result<()> {
        Ok(digital::set_output(node, value))
    }
    fn digital_get_din00(&self,node:i32) ->Result<bool> {
        let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(0).unwrap())
    }
    fn digital_get_din01(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(1).unwrap())
    }
    fn digital_get_din02(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(2).unwrap())
    }
    fn digital_get_din03(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(3).unwrap())
    }
    fn digital_get_din04(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(4).unwrap())
    }
    fn digital_get_din05(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(5).unwrap())
    }
    fn digital_get_din06(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(6).unwrap())
    }
    fn digital_get_din07(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(7).unwrap())
    }
    fn digital_get_din08(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(8).unwrap())
    }
    fn digital_get_din09(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(9).unwrap())
    }
    fn digital_get_din10(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(10).unwrap())
    }
    fn digital_get_din11(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(11).unwrap())
    }
    fn digital_get_din12(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(12).unwrap())
    }
    fn digital_get_din13(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(13).unwrap())
    }
    fn digital_get_din14(&self,node:i32) ->Result<bool> {
         let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(14).unwrap())
    }
    fn digital_get_din15(&self,node:i32) ->Result<bool> {
        let din = self.digital_get_din(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(15).unwrap())
    }
    fn digital_get_dout00(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(0).unwrap())
    }
    fn digital_get_dout01(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(1).unwrap())
    }
    fn digital_get_dout02(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(2).unwrap())
    }
    fn digital_get_dout03(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(3).unwrap())
    }
    fn digital_get_dout04(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(4).unwrap())
    }
    fn digital_get_dout05(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(5).unwrap())
    }
    fn digital_get_dout06(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(6).unwrap())
    }
    fn digital_get_dout07(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(7).unwrap())
    }
    fn digital_get_dout08(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(8).unwrap())
    }
    fn digital_get_dout09(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(9).unwrap())
    }
    fn digital_get_dout10(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(10).unwrap())
    }
    fn digital_get_dout11(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(11).unwrap())
    }
    fn digital_get_dout12(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(12).unwrap())
    }
    fn digital_get_dout13(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(13).unwrap())
    }
    fn digital_get_dout14(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(14).unwrap())
    }
    fn digital_get_dout15(&self,node:i32) ->Result<bool> {
        let dout = self.digital_get_dout(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(15).unwrap())
    }

    fn digital_set_dout00(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(0,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout01(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(1,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout02(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(2,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout03(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(3,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout04(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(4,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout05(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(5,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout06(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(6,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout07(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(7,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout08(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(8,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout09(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(9,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout10(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(10,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout11(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(11,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout12(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(12,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout13(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(13,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout14(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(14,value);
        self.digital_set_dout(node,dout)
    }
    fn digital_set_dout15(&self,node:i32,value:bool) -> Result<()>{
        let mut dout:u16 = self.digital_get_dout(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(15,value);
        self.digital_set_dout(node,dout)
    }


}

// pub struct  Can4Rpc;

