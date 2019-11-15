/// Node Analog extension
/// 
/// **Detals**
/// Unterstutzte indexen:
/// * `0x6411`    - SubNumber=21 ParameterName=Write Analog Output 16 Bit ObjectType=0x8 
///     - `0x00`  - ParameterName=Number of Elements ObjectType=0x7 DataType=0x0005 DefaultValue=20 AccessType=ro 
///     - `0x01`  - ParameterName=AnalogueOutput16[01] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x02`  - ParameterName=AnalogueOutput16[02] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x03`  - ParameterName=AnalogueOutput16[03] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x04`  - ParameterName=AnalogueOutput16[04] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x05`  - ParameterName=AnalogueOutput16[05] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x06`  - ParameterName=AnalogueOutput16[06] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x07`  - ParameterName=AnalogueOutput16[07] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x08`  - ParameterName=AnalogueOutput16[08] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x09`  - ParameterName=AnalogueOutput16[09] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x0A`  - ParameterName=AnalogueOutput16[10] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x0B`  - ParameterName=AnalogueOutput16[11] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x0C`  - ParameterName=AnalogueOutput16[12] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x0D`  - ParameterName=AnalogueOutput16[13] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x0E`  - ParameterName=AnalogueOutput16[14] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x0F`  - ParameterName=AnalogueOutput16[15] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x10`  - ParameterName=AnalogueOutput16[16] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x11`  - ParameterName=AnalogueOutput16[17] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x12`  - ParameterName=AnalogueOutput16[18] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x13`  - ParameterName=AnalogueOutput16[19] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1 
///     - `0x14`  - ParameterName=AnalogueOutput16[20] ObjectType=0x7 DataType=0x0003 AccessType=rww PDOMapping=1
// use futures::prelude::*;
use super::error::CanError;
use super::can::Message;
use super::CAN;
use std::fmt;

pub type AIOext = Vec<u16>;

/// AnalogExtNode 
pub struct AnalogextNode {
    pub node:  u32, 
    pub index: u16,
    pub outs:  Vec<u16>,
}

impl Default for AnalogextNode {
    fn default()  -> Self {
        Self {
            node:  0x1C,
            index: 0x6411,
            outs:  Vec::new(),
           
        }
    }
}

impl AnalogextNode {
    pub fn new(node:u32) -> AnalogextNode {
        Self {
            node:  node,
            index: 0x6411,
            outs:  Vec::new(),
        }
    }
    pub async fn init_outs(&mut self) -> Result<usize,CanError> {
        let tx = Message::new_message(self.node,6411,0x0,Vec::new())?;
        let rx = CAN::read(tx)?;
        let outs = rx.to_u8() as usize;
        self.outs = Vec::with_capacity(outs);
        Ok(outs)
    }

    pub async fn read(&mut self, num: usize) -> Result<u16,CanError> {
        if num >= self.outs.len() || num == 0 {
            return Err(CanError::AnalogextNode{msg: format!("on read from [0x{:02X}:0x{:04X}:0x{:02X}]:[0x{:02X}] -> subindex[0x{:02X}] not available",self.node,self.index,num,self.outs.len(),num)})        }
        let tx = Message::new_message(self.node,6411,num as u8,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.outs[num] = rx.to_u16();
        Ok(self.outs[num])
    }
    pub async fn write(&mut self, num: usize, value: u16) -> Result<(),CanError> {
        if num > self.outs.len() || num == 0 {
            return Err(CanError::AnalogextNode{msg: format!("on write 0X{:02X} to [0x{:02X}:0x{:04X}:0x{:02X}]:[0x{:02X}] -> subindex[0x{:02X}] not available",value,self.node,self.index,num,self.outs.len(),num)})
        }
        let tx = Message::new_u16(self.node,6411,num as u8,value)?;
        let _ =  CAN::write(tx)?;
        self.outs[num] = value;
        Ok(())
    }
}



