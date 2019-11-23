use serde::{Deserialize,Serialize};

pub use super::Msg;
pub use super::analog::Analog;
pub use super::aouts::AOuts;
pub use super::digital::Digital;
pub use super::motor::DoppelMotor;
use bitvec::prelude::*;
use crate::can;
use can::{SharedCan};
use crate::error::CanError;
// use crate::rpc::motor as motor1;
// use crate::rpc::motor as motor2;
use can::*;
use jsonrpc_core::{Result};

use lazy_static::lazy_static;

lazy_static!{
    static ref CANBUS: SharedCan = SharedCan::open("vcan0").unwrap();
}

pub fn candevice() -> Result<Can> {
    let can = Can::open("vcan0")?;
    Ok(can)
}
// fn read_data(addr:&Addr) -> Result<Data,CanError> {
    // let data = CANBUS.read(addr)?;
    // Ok(data)
// }

// fn write_data(iface:String,addr:Addr,data:Data) -> Result<(),CanError> {
//     let mut canbus = CANBUS.lock().unwrap();
//     canbus.entry(iface).or_insert(Can::open(&iface)?).write(addr,data)?;
//     incomming(addr,data);
//     // let data = can.read(addr)?;
//     Ok(())
// }

pub fn can0_read(addr:&Addr) -> Result<Data>{
    let data = candevice()?.read(addr)?;
    Ok(data)
}
fn can0_write(addr:&Addr,data:&Data) -> Result<()>{
    candevice()?.write(addr,data)?;
    Ok(())
}

pub struct CanNode(&'static Can);

impl CanNode {
    pub fn new(can: &'static Can) -> Self {
        CanNode(can)
    }
}

impl Msg for CanNode {
    fn read_data(&self,node:u32,index:u16,sub:u8) -> Result<Data> {
        let data = can0_read(&Addr::new(node,index,sub))?;
        Ok(data)
    }
    fn write_data(&self,node:u32,index:u16,sub:u8,data: Data) -> Result<()>{
		can0_write(&Addr::new(node,index,sub),&data)?;
		Ok(())
    }
}

// pub struct CanBus(&'a Can)

pub struct AnalogNode;

// impl AnalogNode {
//     pub fn new(can:SharedCan) -> Self {
//         AnalogNode(can)
//     }
// }

impl Analog for AnalogNode {
    fn analog_get_in01(&self,node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6100,0x1))?;
        Ok(data.into())
    }
    fn analog_get_in02(&self,node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6100,0x2))?;
        Ok(data.into())
    }
    fn analog_get_in03(&self,node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6100,0x3))?;
        Ok(data.into())
    }
    fn analog_get_in04(&self,node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6100,0x4))?;
        Ok(data.into())
    }
    fn analog_get_in05(&self,node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6110,0x1))?;
        Ok(data.into())
    }
    fn analog_get_out(&self,node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6111,0x1))?;
        Ok(data.into())
    }
    fn analog_set_out(&self,node:u32,value: u16) -> Result<()> {
        can0_write(&Addr::new(node,0x6111,1),&value.into())?;
        Ok(())
    }
    fn analog_get_temp01(&self,node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6021,0x1))?;
        Ok(data.into())
    }
    fn analog_get_temp02(&self,node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6021,0x1))?;
        Ok(data.into())
    }
    fn analog_get_temp03(&self,node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6021,0x3))?;
        Ok(data.into())
    }
    fn analog_get_uart01(&self,node:u32) -> Result<Vec<u8>> {
        let data = can0_read(&Addr::new(node,0x6000,0x1))?;
        Ok(data.into())
    }
    fn analog_get_uart02(&self,node:u32) -> Result<Vec<u8>> {
        let data = can0_read(&Addr::new(node,0x6010,0x1))?;
        Ok(data.into())
    }
    fn analog_set_uart01(&self,node:u32,data:Vec<u8>) -> Result<()> {
        can0_write(&Addr::new(node,0x6000,0x1),&data.into())?;
        Ok(())
    }
    fn analog_set_uart02(&self,node:u32,data:Vec<u8>) -> Result<()> {
        can0_write(&Addr::new(node,0x6010,0x1),&data.into())?;
        Ok(())
    }
}


pub struct AnalogOutputs;

// impl AnalogOutputs {
    // pub fn new(can: &'static Can) -> Self {
        // Self(can)
    // }
// }

impl AOuts for AnalogOutputs{
    fn outs_count(&self) -> Result<u8> {
        let data = can0_read(&Addr::new(0x1c,0x6411,0))?;
        Ok(data.into())
    }
    fn get_outs(&self, num: u8) -> Result<u16> {
        let data = can0_read(&Addr::new(0x1c,0x6411,num))?;
        Ok(data.into())
    }
    fn set_outs(&self, num: u8, val: u16) -> Result<()>{
        can0_write(&Addr::new(0x1c,0x6411,num),&val.into())?;
        Ok(())
    }
}

pub struct DigitalNode;

// impl DigitalNode {
    // pub fn new(can: &'static Can) -> Self {
        // Self(can)
    // }
// }

impl Digital for DigitalNode {
    fn get_info(&self,node:u32) -> Result<String> {
        Ok(format!("Digitan Node {}",node))
    }
    fn get_inputs(&self, node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6100,0x1))?;
        Ok(data.into())
    }
    fn get_outputs(&self, node:u32) ->Result<u16> {
        let data = can0_read(&Addr::new(node,0x6101,0x1))?;
        Ok(data.into())
    }
    fn set_outputs(&self,node:u32,value:u16) -> Result<()>{
        can0_write(&Addr::new(node,0x6101,0x1),&value.into())?;
        Ok(())
    }
    fn get_input00(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(0).unwrap())
    }
    fn get_input01(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(1).unwrap())
    }
    fn get_input02(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(2).unwrap())
    }
    fn get_input03(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(3).unwrap())
    }
    fn get_input04(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(4).unwrap())
    }
    fn get_input05(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(5).unwrap())
    }
    fn get_input06(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(6).unwrap())
    }
    fn get_input07(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(7).unwrap())
    }
    fn get_input08(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(8).unwrap())
    }
    fn get_input09(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(9).unwrap())
    }
    fn get_input10(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(10).unwrap())
    }
    fn get_input11(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(11).unwrap())
    }
    fn get_input12(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(12).unwrap())
    }
    fn get_input13(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(13).unwrap())
    }
    fn get_input14(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(14).unwrap())
    }
    fn get_input15(&self, node:u32) ->Result<bool> {
        let din = self.get_inputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(15).unwrap())
    }
    fn get_output00(&self, node:u32) ->Result<bool> {
        let din = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(0).unwrap())
    }
    fn get_output01(&self, node:u32) ->Result<bool> {
        let din = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(1).unwrap())
    }
    fn get_output02(&self, node:u32) ->Result<bool> {
        let din = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(2).unwrap())
    }
    fn get_output03(&self, node:u32) ->Result<bool> {
        let din = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(din);
        Ok(bv.get(3).unwrap())
    }
    fn get_output04(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(4).unwrap())
    }
    fn get_output05(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(5).unwrap())
    }
    fn get_output06(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(6).unwrap())
    }
    fn get_output07(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(7).unwrap())
    }
    fn get_output08(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(8).unwrap())
    }
    fn get_output09(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(9).unwrap())
    }
    fn get_output10(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(10).unwrap())
    }
    fn get_output11(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(11).unwrap())
    }
    fn get_output12(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(12).unwrap())
    }
    fn get_output13(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(13).unwrap())
    }
    fn get_output14(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(14).unwrap())
    }
    fn get_output15(&self, node:u32) ->Result<bool> {
        let dout = self.get_outputs(node)?;
        let bv  = BitVec::<BigEndian, u16>::from_element(dout);
        Ok(bv.get(15).unwrap())
    }
    fn set_output00(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(0,value);
        self.set_outputs(node,dout)
    }
    fn set_output01(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(1,value);
        self.set_outputs(node,dout)
    }
    fn set_output02(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(2,value);
        self.set_outputs(node,dout)
    }
    fn set_output03(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(3,value);
        self.set_outputs(node,dout)
    }
    fn set_output04(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(4,value);
        self.set_outputs(node,dout)
    }
    fn set_output05(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(5,value);
        self.set_outputs(node,dout)
    }
    fn set_output06(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(6,value);
        self.set_outputs(node,dout)
    }
    fn set_output07(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(7,value);
        self.set_outputs(node,dout)
    }
    fn set_output08(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(8,value);
        self.set_outputs(node,dout)
    }
    fn set_output09(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(9,value);
        self.set_outputs(node,dout)
    }
    fn set_output10(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(10,value);
        self.set_outputs(node,dout)
    }
    fn set_output11(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(11,value);
        self.set_outputs(node,dout)
    }
    fn set_output12(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(12,value);
        self.set_outputs(node,dout)
    }
    fn set_output13(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(13,value);
        self.set_outputs(node,dout)
    }
    /// Set Digital Output
    fn set_output14(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(14,value);
        self.set_outputs(node,dout)
    }
    /// Set Digital Output
    fn set_output15(&self, node:u32,value:bool) -> Result<()> {
        let mut dout:u16 = self.get_outputs(node)?;
        let bv:&mut BitSlice<BigEndian,u16> = dout.as_mut_bitslice();
        bv.set(15,value);
        self.set_outputs(node,dout)
    }
}



pub struct MotorNode;

// impl MotorNode {
//     pub fn new(can: &'static Can) -> Self {
//         Self(can)
//     }
// }

/// Motor oder Doppelmotor
/// 6X00:2 ParameterName=Endschalter, geschlossen=1 ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=0 PDOMapping=0
/// 6X00:3 ParameterName=Endschalter invertieren ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
/// 6X01:1 ParameterName=Command / Status 0/1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
/// 6X01:2 ParameterName=Command  go to Pos ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
/// 6X01:4 - ParameterName=Max Position ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1800 PDOMapping=0
/// 6X01:5 - ParameterName=Fahrparameter ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
/// 6X01:6 - ParameterName=Fahrparameter ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
///  Addressen:
///  `6101` - ParameterName=Stepper 1 - Position ObjectType=0x8 SubNumber=5
///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=5 PDOMapping=0
///       1 - ParameterName=Command / Status 0/1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///       2 - ParameterName=Command  go to Pos ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
///       3 - ParameterName=Position Old ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
///       4 - ParameterName=Max Position ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1800 PDOMapping=0
///       5 - ParameterName=Fahrparameter ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
// 
//
/// 
///  Typed:
///     0x001 - bool
///     0x002 - u8
///     0x003 - u16
///     0x004 - i16
///     0x006 - i16

impl DoppelMotor for MotorNode {
    fn get_info(&self,node:u32)-> Result<String>  {
        Ok(format!("DoppelMotor Node {}",node))
    }
    fn get_uart01(&self,node:u32)-> Result<Vec<u8>> {
        let data = can0_read(&Addr::new(node,0x6000,0x1))?;
        Ok(data.into())
    }

    fn get_uart02(&self,node:u32)-> Result<Vec<u8>> {
        let data = can0_read(&Addr::new(node,0x6010,0x1))?;
        Ok(data.into())
    }
    fn set_uart01(&self,node:u32,data:Vec<u8>)-> Result<()> {
        can0_write(&Addr::new(node,0x6000,0x1),&data.into())?;
        Ok(())
    }
    fn set_uart02(&self,node:u32,data:Vec<u8>)-> Result<()> {
        can0_write(&Addr::new(node,0x6010,0x1),&data.into())?;
        Ok(())
    }
    fn set_baut01(&self,node:u32,bautrate:u32)-> Result<()> {
        can0_write(&Addr::new(node,0x6000,0x4),&bautrate.into())?;
        Ok(())
    }
    fn set_baut02(&self,node:u32,bautrate:u32)-> Result<()> {
        can0_write(&Addr::new(node,0x6010,0x4),&bautrate.into())?;
        Ok(())
    }

    /// A  `6100` - ParameterName=Stepper 1 / Ruhrer 1 ObjectType=0x8 SubNumber=4
    ///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=3 PDOMapping=0
    ///       4 - ParameterName=Diagnose TCM249 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
    /// B  `6200` - ParameterName=Stepper 2 / Ruhrer 2 ObjectType=0x8 SubNumber=4
    ///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=3 PDOMapping=0
    ///     rpc interface nicht vorhanden
    ///       4 - ParameterName=Diagnose TCM249 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
    /// 
    /// A 0x1 ParameterName=Stepper=0, Stirrer =1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
    fn set_option1(&self,node:u32,value:u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6100,0x1),&value.into())?;
        Ok(())
    }
    /// B 0x1 ParameterName=Stepper=0, Stirrer =1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
    fn set_option2(&self,node:u32,value:u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6200,0x1),&value.into())?;
        Ok(())
    }
    /// A 0x2 - ParameterName=Endschalter, geschlossen=1 ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=0 PDOMapping=0
    fn get_endschalter1(&self,node:u32) -> Result<u8>{
        let data = can0_read(&Addr::new(node,0x6100,0x2))?;
        Ok(data.into())
    }
    /// B 0x2 - ParameterName=Endschalter, geschlossen=1 ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=0 PDOMapping=0
    fn get_endschalter2(&self,node:u32) -> Result<u8> {
        let data = can0_read(&Addr::new(node,0x6200,0x2))?;
        Ok(data.into())
         
    }
    /// A 0x2 - ParameterName=Endschalter, geschlossen=1 ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=0 PDOMapping=0
    fn invert_endschalter1(&self,node:u32,value:u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6100,0x3),&value.into())?;
        Ok(())
    }
    /// B 0x2 - ParameterName=Endschalter, geschlossen=1 ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=0 PDOMapping=0
    fn invert_endschalter2(&self,node:u32,value:u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6200,0x3),&value.into())?;
        Ok(())
    }

    /// A `6101` - ParameterName=Stepper 1 - Position ObjectType=0x8 SubNumber=5
    ///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=5 PDOMapping=0
    /// B `6201` - ParameterName=Stepper 2 - Position ObjectType=0x8 SubNumber=5
    ///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=5 PDOMapping=0
    /// 
    ///   rpc interface nicht vorchanden 
    ///     3 - ParameterName=Position Old ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
    ///     6 - ParameterName=Stall guard flag ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
    /// 
    /// set command
    ///   wo 1- , 2 - , 3 - , 4 - 
    /// A 0x1 - ParameterName=Command / Status 0/1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
    fn set_command1(&self,node:u32,cmd:u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6101,0x1),&cmd.into())?;
        Ok(())
    }
    /// B 0x1 - ParameterName=Command / Status 0/1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
    fn set_command2(&self,node:u32,cmd:u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6201,0x1),&cmd.into())?;
        Ok(())
    }
    /// A 0x2 - ParameterName=Command  go to Pos ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
    fn goto_position1(&self,node:u32,pos:u16) -> Result<()> {
        can0_write(&Addr::new(node,0x6101,0x2),&pos.into())?;
        Ok(())
    }
    /// B 0x2 - ParameterName=Command  go to Pos ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
    fn goto_position2(&self,node:u32,pos:u16) -> Result<()> {
        can0_write(&Addr::new(node,0x6201,0x2),&pos.into())?;
        Ok(())
    }
    /// A 0x3 - ParameterName=Command  go to Pos ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
    fn get_position1(&self,node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6101,0x2))?;
        Ok(data.into())
         
    }
    /// B 0x3 - ParameterName=Command  go to Pos ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
    fn get_position2(&self,node:u32) -> Result<u16> {
        let data = can0_read(&Addr::new(node,0x6201,0x2))?;
        Ok(data.into())
         
    }
    /// A 0x4 - ParameterName=Max Position ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1800 PDOMapping=0
    fn set_max_position1(&self,node:u32,max:u16) -> Result<()> {
        can0_write(&Addr::new(node,0x6101,0x4),&max.into())?;
        Ok(())
    }
    /// B 0x4 - ParameterName=Max Position ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1800 PDOMapping=0
    fn set_max_position2(&self,node:u32,max:u16) -> Result<()> {
        can0_write(&Addr::new(node,0x6201,0x4),&max.into())?;
        Ok(())
    }
    /// A 0x5  - ParameterName=Fahrparameter ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
    fn set_velocity1(&self,node:u32,velocity:u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6101,0x5),&velocity.into())?;
        Ok(())
    }
    /// B 0x5  - ParameterName=Fahrparameter ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
    fn set_velocity2(&self,node:u32,velocity:u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6201,0x5),&velocity.into())?;
        Ok(())
    }

    /// A `6103` - ParameterName=Strom 1 ObjectType=0x8 SubNumber=5
    ///      `0` - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=4 PDOMapping=0
    /// B `6203` - ParameterName=Strom 1 ObjectType=0x8 SubNumber=5
    ///      `0` - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=4 PDOMapping=0
    /// 
    ///  rpc - nicht vorchanden 
    ///      `2` - ParameterName=Stromwert Digital ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=204 PDOMapping=0
    ///      `3` - ParameterName=Haltestromsollwert / mA ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=150 PDOMapping=0
    ///      `4` - ParameterName=Haltestromwert Digital ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=51 PDOMapping=0
    /// 
    /// set current for A
    /// A `0x1` - ParameterName=Stromsollwert / mA ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=600 PDOMapping=0
    fn set_stromsollwert1(&self,node:u32,value:u16) -> Result<()> {
        can0_write(&Addr::new(node,0x6103,0x1),&value.into())?;
        Ok(())
    }

    /// set current for B
    /// B `0x1` - ParameterName=Stromsollwert / mA ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=600 PDOMapping=0
    fn set_stromsollwert2(&self,node:u32,value:u16) -> Result<()> {
        can0_write(&Addr::new(node,0x6203,0x1),&value.into())?;
        Ok(())
    }
    /// A `6102` - ParameterName=Ruhrer 1 ObjectType=0x8 SubNumber=3
    ///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=2 PDOMapping=0
    ///       2 - ParameterName=Delay ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=-126 PDOMapping=0
    /// A `6202` - ParameterName=Ruhrer 1 ObjectType=0x8 SubNumber=3
    ///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=2 PDOMapping=0
    /// 
    /// start|stop stirrer1
    /// A      1 - ParameterName=Aus=0, Ein=1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
    fn set_stirrer1(&self,node:u32,value: u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6102,0x1),&value.into())?;
        Ok(())
    }
    /// start|stop stirrer2
    /// b      1 - ParameterName=Aus=0, Ein=1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
     fn set_stirrer2(&self,node:u32,value: u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6202,0x1),&value.into())?;
        Ok(())
    }
    /// A   2 - ParameterName=Delay ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=-126 PDOMapping=0
    fn set_delay1(&self,node:u32,delay:u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6102,0x2),&delay.into())?;
        Ok(())
    }
    /// A   2 - ParameterName=Delay ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=-126 PDOMapping=0
    fn set_delay2(&self,node:u32,delay:u8) -> Result<()> {
        can0_write(&Addr::new(node,0x6202,0x2),&delay.into())?;
        Ok(())
    }
}
