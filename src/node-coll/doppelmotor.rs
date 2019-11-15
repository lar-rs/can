use futures::prelude::*;
use super::can::{ Message };
use serde_derive::{Deserialize,Serialize};
use super::CAN;
use super::error::CanError;

/// Motor oder Doppelmotor
///  Typed:
///     0x001 - bool
///     0x002 - u8
///     0x003 - u16
///     0x004 - i16
///     0x006 - i16
///  Addressen:
///  `6100` - ParameterName=Stepper 1 / Ruhrer 1 ObjectType=0x8 SubNumber=4
///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=3 PDOMapping=0
///       1 - ParameterName=Stepper=0, Ruhrer =1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
///       2 - ParameterName=Endschalter, geschlossen=1 ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=0 PDOMapping=0
///       3 - ParameterName=Endschalter invertieren ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///       4 - ParameterName=Diagnose TCM249 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///  `6101` - ParameterName=Stepper 1 - Position ObjectType=0x8 SubNumber=5
///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=5 PDOMapping=0
///       1 - ParameterName=Command / Status 0/1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///       2 - ParameterName=Command  go to Pos ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
///       3 - ParameterName=Position Old ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
///       4 - ParameterName=Max Position ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1800 PDOMapping=0
///       5 - ParameterName=Fahrparameter ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
///       6 - ParameterName=Stall guard flag ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
///  `6102` - ParameterName=Ruhrer 1 ObjectType=0x8 SubNumber=3
///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=2 PDOMapping=0
///       1 - ParameterName=Aus=0, Ein=1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
///       2 - ParameterName=Delay ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=-126 PDOMapping=0
///  `6103` - ParameterName=Strom 1 ObjectType=0x8 SubNumber=5
///      `0` - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=4 PDOMapping=0
///      `1` - ParameterName=Stromsollwert / mA ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=600 PDOMapping=0
///      `2` - ParameterName=Stromwert Digital ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=204 PDOMapping=0
///      `3` - ParameterName=Haltestromsollwert / mA ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=150 PDOMapping=0
///      `4` - ParameterName=Haltestromwert Digital ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=51 PDOMapping=0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stepper {
    node: u32,
/// Mode Stepper,Stirrer
    pub index: u16,
    pub state: u8,
    pub endshalter: u8,
    pub invert_endshalter: u8,
    pub command: u8,
    pub position: i16,
    pub position_old: i16,
    pub max_pos: i16,
    pub parameter: u8,
    pub stall_guard: u8,
    pub stirrer_run: u8,
    pub delay:i8,
    pub stromsolwert: i16,
    pub stromwert_digital:i16,
    pub haltestromsollwert: i16,
    pub haltestromsollwert_digital: i16,
}

impl Default for Motor {
    fn default() -> Self {
        Self {
            node: 0x12,
            index:6100,
            state: 1,
            endshalter: 0,
            invert_endshalter:0,
            command: 0,
            position: 0,
            position_old: 0,
            max_pos: 1800,
            parameter: 1,
            stall_guard: 1,
            stirrer_run: 0,
            delay:-126,
            stromsolwert: 600,
            stromwert_digital:204,
            haltestromsollwert: 150,
            haltestromsollwert_digital:51,
        }
    }
}


impl Motor {
    fn new(node: u32, index: u16) -> Self {
        Self {
            node: node,
            index:index,
            state: 1,
            endshalter: 0,
            invert_endshalter:0,
            command: 0,
            position: 0,
            position_old: 0,
            max_pos: 1800,
            parameter: 1,
            stall_guard: 1,
            stirrer_run: 0,
            delay:-126,
            stromsolwert: 600,
            stromwert_digital:204,
            haltestromsollwert: 150,
            haltestromsollwert_digital:51,
        }
    }
    pub fn get_index(&self) -> u16 {
        self.index
    }
/// 6X00:1 ParameterName=Stepper=0, Ruhrer =1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
    pub async fn read_state(&mut self) -> Result<u8,CanError> {
        let tx = Message::new_message(self.node,self.index,0x1,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.state = rx.to_u8();
        Ok(self.state)
    }
    pub async fn set_ruhrer(&mut self)-> Result<(),CanError> {
        let tx = Message::new_u8(self.node,self.index,0x1,1)?;
        let rx = CAN::read(tx)?;
        Ok(())
    }
    pub async fn set_stepper(&mut self)-> Result<(),CanError> {
        let tx = Message::new_u8(self.node,self.index,0x1,0)?;
        let rx = CAN::read(tx)?;
        Ok(())
    }
/// 6X00:2 ParameterName=Endschalter, geschlossen=1 ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=0 PDOMapping=0
    pub async fn read_endshalter(&mut self) -> Result<u8,CanError> {
        let tx = Message::new_message(self.node,self.index,0x2,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.endshalter = rx.to_u8();
        Ok(self.endshalter)
    }
/// 6X00:3 ParameterName=Endschalter invertieren ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
    pub async fn read_invert_endshalter(&mut self) -> Result<u8,CanError> {
        let tx = Message::new_message(self.node,self.index,0x3,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.invert_endshalter = rx.to_u8();
        Ok(self.invert_endshalter)
    }
    pub async fn invert_endshalter(&mut self) -> Result<(),CanError> {
        self.invert_endshalter = if self.invert_endshalter > 0 { 0 } else { 1 };
        let tx = Message::new_u8(self.node,self.index,0x3,self.invert_endshalter)?;
        let _ = CAN::write(tx)?;
        Ok(())
    }
/// 6X01:1 ParameterName=Command / Status 0/1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
    pub async fn read_command(&mut self) -> Result<u8,CanError> {
        let tx = Message::new_message(self.node,self.index+0x1,0x1,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.command= rx.to_u8();
        Ok(self.command)
    }
    pub async fn write_command(&mut self,command:u8) -> Result<(),CanError> {
        self.command = command;
        let tx = Message::new_u8(self.node,self.index,0x3,self.command)?;
        let _ = CAN::write(tx)?;
        Ok(())
    }
/// 6X01:2 ParameterName=Command  go to Pos ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
    pub async fn read_position(&mut self) -> Result<i16,CanError> {
        let tx = Message::new_message(self.node,self.index+0x1,0x2,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.position= rx.to_i16();
        Ok(self.position)
    }
/// 6X01:3 ParameterName=Position Old ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
    pub async fn read_position_old(&mut self) -> Result<i16,CanError> {
        let tx = Message::new_message(self.node,self.index+0x1,0x3,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.position_old = rx.to_i16();
        Ok(self.position_old)
    }
/// 6X01:4 - ParameterName=Max Position ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1800 PDOMapping=0
    pub async fn read_max_pos(&mut self) -> Result<i16,CanError> {
        let tx = Message::new_message(self.node,self.index+0x1,0x4,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.position= rx.to_i16();
        Ok(self.position)
    }
    pub async fn write_max_position(&mut self,max:u16) -> Result<u8,CanError> {
        let tx = Message::new_u16(self.node,self.index+0x1,0x1,max)?;
        let rx = CAN::read(tx)?;
        self.command= rx.to_u8();
        Ok(self.command)
    }
/// 6X01:5 - ParameterName=Fahrparameter ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
    pub async fn read_fahrparameter(&mut self) -> Result<u8,CanError> {
        let tx = Message::new_message(self.node,self.index+0x1,0x5,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.parameter = rx.to_u8();
        Ok(self.parameter)
    }
    pub async fn write_fahrparameter(&mut self,par:u8) -> Result<(),CanError> {
        let tx = Message::new_u8(self.node,self.index+0x1,0x1,par)?;
        let rx = CAN::read(tx)?;
        self.parameter = rx.to_u8();
        Ok(())
    }
/// 6X01:6 - ParameterName=Fahrparameter ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
    pub async fn read_stall_guard(&mut self) -> Result<u8,CanError> {
        let tx = Message::new_message(self.node,self.index+0x1,0x6,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.parameter = rx.to_u8();
        Ok(self.parameter)
    }
    pub async fn read_all(&mut self) -> Result<(),CanError> {
        self.read_state().await?;
        // self.
        Ok(())
    }

}

/// Pump controlling.
/// * 6110 ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=8 PDOMapping=0
///     - 1 ParameterName=0=Stop, 1=Start 2=Wait ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 2 ParameterName=0=Normal, 1=Spulen,2=Intervall ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 3 ParameterName=Drehrichtung rechts / links ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 4 ParameterName=Speed soll ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 5 ParameterName=Interwall Pos-Impulse ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 6 ParameterName=Interwall Time / sec ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 7 ParameterName=Position ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
///     - 8 ParameterName=Delay PW ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
/// * 6111  ParameterName=Konstanten 1 ObjectType=0x8 SubNumber=4
///     - 0  ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=3 PDOMapping=0
///     - 1  ParameterName=K-p ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1 PDOMapping=0
///     - 2  ParameterName=K-d ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1 PDOMapping=0
///     - 3  ParameterName=K-i ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1 PDOMapping=0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pump {
    /// Node id
    node: u32,
    /// Index 6110=Part1 6210=Part2
    pub index: u16,
    /// Status 0=Stop, 1=Start 2=Wait
    pub status: u8,
    /// Runstate 0=Normal, 1=Spulen,2=Intervall
    pub runstate: u8,
    pub drehrichtung: u8,
    pub speed:u8,
    pub interval_impuls: i16,
    pub interval_time:i16,
    pub position:i16,
    pub delay:i16,
    pub cons_kp:i16,
    pub cons_kd:i16,
    pub cons_ki:i16,
}
impl Default for Pump {
    fn default() -> Self{
        Self {
            node: 0x12,
            index:6110,
            status: 0,
            runstate:0,
            drehrichtung: 0,
            speed:0,
            interval_impuls:0,
            interval_time:0,
            position:0,
            delay:0,
            cons_kp:0,
            cons_kd:0,
            cons_ki:0,

        }
    }
}


impl Pump {
    fn new(node:u32,index:u16) ->Self {
        Self {
            node: node,
            index:index,
            status: 0,
            runstate:0,
            drehrichtung: 0,
            speed:0,
            interval_impuls:0,
            interval_time:0,
            position:0,
            delay:0,
            cons_kp:0,
            cons_kd:0,
            cons_ki:0,

        }
    }
///6X10:1 ParameterName=0=Stop, 1=Start 2=Wait ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
    pub async fn read_status(&mut self) -> Result<u8,CanError> {
        let tx = Message::new_message(self.node,self.index,0x1,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.status = rx.to_u8();
        Ok(self.status)
    }
///6X10:2   - 2 ParameterName=0=Normal, 1=Spulen,2=Intervall ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
    pub async fn read_runstatus(&mut self) -> Result<u8,CanError> {
        let tx = Message::new_message(self.node, self.index, 0x2, Vec::new())?;
        let rx = CAN::read(tx)?;
        self.runstate = rx.to_u8();
        Ok(self.runstate)
    }
}

// [6000] ParameterName=Uart 0 ObjectType=0x8 SubNumber=4
// [6000sub0] ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=6 PDOMapping=0
// [6000sub1] ParameterName=Uart Daten1 ObjectType=0x7 DataType=0x0009 AccessType=rw DefaultValue= PDOMapping=0
// [6000sub2] ParameterName=RX-Length ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=126 PDOMapping=0
// [6000sub3] ParameterName=TX-Length ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=126 PDOMapping=0
// [6000sub4] ParameterName=Set Baudrate ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=9 PDOMapping=0
// [6010] ParameterName=Uart 1 ObjectType=0x8 SubNumber=4
// [6010sub0] ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=6 PDOMapping=0
// [6010sub1] ParameterName=Uart Daten2 ObjectType=0x7 DataType=0x0009 AccessType=rw DefaultValue= PDOMapping=0
// [6010sub2] ParameterName=RX-Length ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=126 PDOMapping=0
// [6010sub3] ParameterName=TX-Length ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=126 PDOMapping=0
// [6010sub4] ParameterName=Set Baudrate ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=9 PDOMapping=0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uart {
    node : u32,
    pub index: u16,
    pub data : Vec<u8>,
    pub baut : u8,
}

impl Default for Uart {
    fn default() -> Self{
        Self {
            node: 0x12,
            index:6000,
            data: Vec::new(),
            baut: 9
        }
    }
}

impl Uart {
    fn new(node:u32,index:u16) ->Self {
        Self {
            node: node,
            index:index,
            data: Vec::new(),
            baut: 9
        }
    }
    pub async fn set_bautrate(&mut self,baut:u8) -> Result<(),CanError> {
        let tx = Message::new_u8(self.node,self.index,0x1,baut)?;
        let _ = CAN::read(tx)?;
        self.baut = baut;
        Ok(())

    }
    pub async fn read_uart01(&mut self) -> Result<Vec<u8>,CanError> {
        let tx = Message::new_message(self.node,self.index,0x1,Vec::new())?;
        let rx = CAN::read(tx)?;
        self.data = rx.get_data();
        Ok(self.data.clone())
    }
}
// [6100sub1]
// ParameterName=Stepper=0, Ruhrer =1
// ObjectType=0x7
// DataType=0x0002
// AccessType=rw
// DefaultValue=1
// PDOMapping=0
// [6100sub2]
// ParameterName=Endschalter, geschlossen=1
// ObjectType=0x7
// DataType=0x0002
// AccessType=ro
// DefaultValue=0
// PDOMapping=0
pub struct DoppelmotorNode{
    /// Index 6100 ,6101
    pub node: u32,
    pub motor1: Motor,
    pub motor2: Motor,
    pub pump1: Pump,
    pub pump2: Pump,
    pub uart1: Uart,
    pub uart2: Uart,
}


impl Default for DoppelmotorNode {
    fn default() -> Self{
        let node = 0x12 as u32;
        let motor1 = Motor::new(0x12,6100);
        let motor2 = Motor::new(0x12,6200);
        let pump1  = Pump::new(0x12,6110);
        let pump2  = Pump::new(0x12,6210);
        let uart1  = Uart::new(0x12,6000);
        let uart2  = Uart::new(0x12,6010);
        Self {
            node,motor1,motor2,pump1,pump2,uart1,uart2
        }
    }
}


impl DoppelmotorNode {
    pub fn new(node : u32) -> DoppelmotorNode {
        DoppelmotorNode {
           node: node,
           motor1: Motor::new(node,6100),
           motor2: Motor::new(node,6200),
           pump1:  Pump::new(node,6110),
           pump2:  Pump::new(node,6210),
           uart1:  Uart::new(node,6000),
           uart2:  Uart::new(node,6010),
        }
    }
    pub async fn read_all (&mut self ) -> Result<bool,CanError> {
        self.motor1.read_all().await?;
        Ok(true)
    }
}


