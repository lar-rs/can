use super::error::CanError;
use serde_derive::{Deserialize,Serialize};
use super::CAN;
use super::can::{
    Message,
};
use bit_vec::BitVec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DInput {
    pub bit: usize,
    pub val: &'static[u8],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalNode {

    pub node: u32,
    pub input: BitVec,
    pub output: BitVec,

}


/// Pin Assignment PL4 ( Outputs Byte 0 )
///
/// Bit Pin  No.   Function
/// 0    1  IN00  Digital Input DC 24V
/// 1    2  IN01  Digital Input DC 24V
/// 2    3  IN02  Digital Input DC 24V
/// 3    4  IN03  Digital Input DC 24V
/// 4    5  IN04  Digital Input DC 24V
/// 5    6  IN05  Digital Input DC 24V
/// 6    7  IN06  Digital Input DC 24V
/// 7    8  IN07  Digital Input DC 24V
/// 8    9  IN10  Digital Input DC 24V
/// 9    10 IN11  Digital Input DC 24V
/// 10   11 IN12  Digital Input DC 24V
/// 11   12 IN13  Digital Input DC 24V
/// 12   13 IN14  Digital Input DC 24V
/// 13   14 IN15  Digital Input DC 24V
/// 14   15 IN16  Digital Input DC 24V
/// 15   16 IN17  Digital Input DC 24V



pub async read_inputs16() -> Result<BitVec,CanError>


pub struct DigitalInput {

}

impl DigitalNode {
    pub fn new(node : u32) -> DigitalNode {
        DigitalNode {
            node : node,
            input: BitVec::from_elem(16, false),
            output:BitVec::from_elem(16, false)
        }
    }
    /// Read all inputs.
    pub async fn read_inputs(&mut self) ->Result<BitVec,CanError> {
        let tx = Message::new_message(self.node,0x6100,0x01,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.input = BitVec::from_bytes(rx.get_data().as_slice());

        Ok(self.input.clone())
    }
    /// Read all outinputs [0..16].
    pub async fn read_outputs(&mut self) -> Result<BitVec,CanError> {
        let tx = Message::new_message(self.node,0x6101,0x01,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.output = BitVec::from_bytes(rx.get_data().as_slice());
        Ok(self.output.clone())
    }
       /// Read all outinputs [0..16].
    pub async fn write_output_bit(&mut self,nbit: usize,val: bool ) -> Result<(),CanError> {
        if nbit > 15 {
            return Err(CanError::DigitalNode{msg: format!("DIO 16bit [0..15]  bit={}",nbit)});
        }
        self.output.set(nbit,val);
        let tx = Message::new_message(self.node,0x6101,0x01,self.output.to_bytes())?;
        let _ = CAN::write(tx)?;
        Ok(())
    }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din00(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(0).ok_or(CanError::DigitalNode{msg: format!("get DI[0] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din01(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(1).ok_or(CanError::DigitalNode{msg: format!("get DI[1] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din02(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(3).ok_or(CanError::DigitalNode{msg: format!("get DI[2] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din03(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(4).ok_or(CanError::DigitalNode{msg: format!("get DI[3] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din04(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(5).ok_or(CanError::DigitalNode{msg: format!("get DI[4] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din05(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(6).ok_or(CanError::DigitalNode{msg: format!("get DI[5] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din06(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(7).ok_or(CanError::DigitalNode{msg: format!("get DI[6] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din07(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(8).ok_or(CanError::DigitalNode{msg: format!("get DI[7] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din10(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(9).ok_or(CanError::DigitalNode{msg: format!("get DI[8] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din11(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(10).ok_or(CanError::DigitalNode{msg: format!("get DI[9] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din12(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(11).ok_or(CanError::DigitalNode{msg: format!("get DI[10] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din13(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(12).ok_or(CanError::DigitalNode{msg: format!("get DI[11] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din14(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(13).ok_or(CanError::DigitalNode{msg: format!("get DI[12] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din15(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(14).ok_or(CanError::DigitalNode{msg: format!("get DI[12] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din16(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(15).ok_or(CanError::DigitalNode{msg: format!("get DI[13] N=0 DInputs[0b{:b}]",digin)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_din18(&mut self) -> Result<bool,CanError> {
    //     let digin = self.read_inputs().await?;

    //     digin.get(15).ok_or(CanError::DigitalNode{msg: format!("get DI[13] N=0 DInputs[0b{:b}]",digin)})
    // }
    //   /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout00(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;
    //     outs.get(0).ok_or(CanError::DigitalNode{msg: format!("get DI[0] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout01(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(1).ok_or(CanError::DigitalNode{msg: format!("get DI[1] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout02(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(2).ok_or(CanError::DigitalNode{msg: format!("get DI[2] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout03(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(3).ok_or(CanError::DigitalNode{msg: format!("get DI[3] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout04(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(4).ok_or(CanError::DigitalNode{msg: format!("get DI[4] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout05(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(5).ok_or(CanError::DigitalNode{msg: format!("get DI[5] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout06(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(6).ok_or(CanError::DigitalNode{msg: format!("get DI[6] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout07(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(7).ok_or(CanError::DigitalNode{msg: format!("get DI[7] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout10(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(8).ok_or(CanError::DigitalNode{msg: format!("get DI[8] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout11(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(9).ok_or(CanError::DigitalNode{msg: format!("get DI[9] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout12(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(10).ok_or(CanError::DigitalNode{msg: format!("get DI[10] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout13(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(11).ok_or(CanError::DigitalNode{msg: format!("get DI[11] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout14(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(12).ok_or(CanError::DigitalNode{msg: format!("get DI[12] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout15(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(13).ok_or(CanError::DigitalNode{msg: format!("get DI[12] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout16(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;

    //     outs.get(14).ok_or(CanError::DigitalNode{msg: format!("get DI[13] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn get_dout17(&mut self) -> Result<bool,CanError> {
    //     let outs = self.read_outputs().await?;
    //     outs.get(15).ok_or(CanError::DigitalNode{msg: format!("get DI[13] N=0 DInputs[0b{:b}]",outs)})
    // }
    //    /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout00(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let bv  = BitVec::from_bytes(&self.output.to_be_bytes());
    //     bv.set(0,val);
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(0).ok_or(CanError::DigitalNode{msg: format!("get DI[0] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout01(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(1).ok_or(CanError::DigitalNode{msg: format!("get DI[1] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout02(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(3).ok_or(CanError::DigitalNode{msg: format!("get DI[2] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout03(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(4).ok_or(CanError::DigitalNode{msg: format!("get DI[3] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout04(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(5).ok_or(CanError::DigitalNode{msg: format!("get DI[4] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout05(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(6).ok_or(CanError::DigitalNode{msg: format!("get DI[5] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout06(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(7).ok_or(CanError::DigitalNode{msg: format!("get DI[6] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout07(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(8).ok_or(CanError::DigitalNode{msg: format!("get DI[7] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout10(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(9).ok_or(CanError::DigitalNode{msg: format!("get DI[8] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout11(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(10).ok_or(CanError::DigitalNode{msg: format!("get DI[9] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout12(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(11).ok_or(CanError::DigitalNode{msg: format!("get DI[10] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout13(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(12).ok_or(CanError::DigitalNode{msg: format!("get DI[11] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout14(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(13).ok_or(CanError::DigitalNode{msg: format!("get DI[12] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout15(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(14).ok_or(CanError::DigitalNode{msg: format!("get DI[12] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout16(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(15).ok_or(CanError::DigitalNode{msg: format!("get DI[13] N=0 DInputs[0b{:b}]",outs)})
    // }
    // /// Digital Input DC 24V (Pin=1 Bit=0 Physical= IN00)
    // pub async fn set_dout18(&mut self , val : bool ) -> Result<bool,CanError> {
    //     let digin = self.write_outputs(outs).await?;

    //     digin.get(15).ok_or(CanError::DigitalNode{msg: format!("get DI[13] N=0 DInputs[0b{:b}]",outs)})
    // }
}



pub mod digital1 {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;
}


pub struct DI01;
pub struct DI02;
pub struct DI03;
pub struct DI04;
pub struct DI05;
pub struct DI06;
pub struct DI07;
pub struct DI08;
pub struct DI10;
pub struct DI11;
pub struct DI12;
pub struct DI13;
pub struct DI14;
pub struct DI15;
pub struct DI16;
pub struct DI17;


pub struct DO01;
pub struct DO02;
pub struct DO03;
pub struct DO04;
pub struct DO05;
pub struct DO06;
pub struct DO07;
pub struct DO08;
pub struct DO10;
pub struct DO11;
pub struct DO12;
pub struct DO13;
pub struct DO14;
pub struct DO15;
pub struct DO16;
pub struct DO17;



#[cfg(test)]
mod tests {
    use super::*;
    /// use socketcan::CANFrame;
    /// Attempt delivery of two messages, using a oneshot channel
    /// to prompt the second message in order to demonstrate that
    /// waiting for CAN reads is not blocking.
    #[test]
    fn test_digital() {
        let board = Can::open("can0").unwrap();
        // board.send_message(Message::read_message(0x18,0x2002,0x0)).unwrap();
        let message = board
            .read_message(Message::read_message(0x18, 0x2002, 0x1))
            .map_err(|err| {
                println!("write io error: {:?}", err);
            })
            .unwrap();
        println!("Message:{:?}", message.to_string());
        board
            .write_message(Message::new_message(0x18, 0x6300, 0x1, vec![0xff, 0x00]))
            .map_err(|err| {
                println!("write io error: {:?}", err);
            })
            .unwrap();
        let d = "FRENZEL+BERG".to_owned().as_bytes().to_vec();
        let message = block!(board.write_message(Message::new_long(0x18, 0x2002, 0x1, d.clone())))
            .map_err(|err| {
                println!("write io error: {:?}", err);
            })
            .unwrap();
        println!("Message:{:?}", message);

        // let message = board.read_message(Message::read_message(0x18,0x2002,0x2)).unwrap();
        // println!("Message:{:?}",message.to_string());
        // let meboard.read_message(Message::read_message(0x18,0x2002,0x2)).unwrap();
        // board.send_message(Message::read_message(0x18,0x2002,0x3)).unwrap();
        // board.send_message(Message::read_message(0x18,0x2002,0x4)).unwrap();
        // board.send_message(Message::read_message(0x18,0x6000,0x1)).unwrap();
        // board.send_message(Message::read_message(0x18,0x6100,0x1)).unwrap();
        // board.send_message(Message::read_message(0x18,0x6300,0x1)).unwrap();
        // board.send_message(Message::read_message(0x18,0x6320,0x1)).unwrap();
        // board.send_message(Message::read_message(0x18,0x6320,0x1)).unwrap();
        // board.write_dout(0).unwrap();
        // assert_eq!(board.read_din().unwrap(),0);
        // assert_eq!(board.read_dout().unwrap(),0);
        // board.write_dout(0b1111100u16).unwrap();
        // assert_eq!(board.read_dout().unwrap(),0b1111100u16);
    }

}
