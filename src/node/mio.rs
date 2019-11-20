use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

use std::{
    sync::{Mutex},
};
use fs2::FileExt;

// use super::error::CanError;
// use once_cell::sync::OnceCell;

// use serde_json::Value;

// use std::collections::HashMap;
// use crossbeam_channel::{bounded, unbounded, Receiver, Sender, TryRecvError};

pub type HID = u64;
pub enum Digital{
    Digital0x18,
    Digital0x19,
    Digital0x1C,
}
pub enum Motor{
    Motor0x12,
    Motor0x13,
}
pub enum Node {
    Analog,
    Motor1,
    Motor2,
    Digital(Digital),
}

lazy_static! {
    // static ref MODS: Mutex<HashMap<u64, MioPin>> = Mutex::new(HashMap::new());
}

// fn check(id:u64) -> bool {
// }

// async get_hidfile() ->


// Key is a tuple (raw key, timestamp).
pub type Key = (u32,u16,u8);

#[derive(Clone, PartialEq)]
pub enum IndexValue {
    Short(u64),
    Long(Vec<u8>),
}



#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MioConfig {
    pub id: u64,
    #[serde(default)]
    pub active_low: bool,
    pub user: Option<String>,
    pub group: Option<String>,
    pub mode: Option<u32>,
}

// impl Mio {
// pub fn can0() -> Result<CANSocket,CanError> {
// INSTANCE.get().expect("logger is not initialized");
// }
// }



pub trait CanBus {
    //"com.lar.service.can", "com/lar/nodes/Analog1" com.lar.nodes.simple
    // fn set_value(&mut self, node: &mut Node) -> Result<(), CanError>;
    // fn get_value(&mut self, node: u32, index: u16, sub: u8) -> Result<Value, CanError>;

    fn get_ain01(&mut self) -> u16;
    fn get_ain02(&mut self) -> u16;
    fn get_ain03(&mut self) -> u16;
    fn get_ain04(&mut self) -> u16;
    fn get_ain05(&mut self) -> u16;
    fn get_aout(&mut self) -> u16;
    fn set_aout(&mut self, val: u16);

    fn get_temp01(&mut self) -> u16;
    fn get_temp02(&mut self) -> u16;
    fn get_temp03(&mut self) -> u16;
    ///com.lar.nodes.Digital16
    fn get_dig18in(&mut self,num:u8) -> bool;
    fn get_dig18out(&mut self,num:u8) -> bool;
    fn set_dig18out(&mut self,num:u8,val:bool);


    ///com.lar.nodes.Digital16
    fn get_dig19in(&mut self,num:u8) -> bool;
    fn get_dig19out(&mut self,num:u8) -> bool;
    fn set_dig19out(&mut self,num:u8, val:bool);

    fn get_uart01(&mut self) -> Vec<u8>;
    fn get_uart02(&mut self) -> Vec<u8>;
    fn get_uart03(&mut self) -> Vec<u8>;
    fn get_uart04(&mut self) -> Vec<u8>;
    fn get_uart05(&mut self) -> Vec<u8>;
    fn get_uart06(&mut self) -> Vec<u8>;

    fn set_uart01(&mut self, data: Vec<u8>);
    fn set_uart02(&mut self, data: Vec<u8>);
    fn set_uart03(&mut self, data: Vec<u8>);
    fn set_uart04(&mut self, data: Vec<u8>);
    fn set_uart05(&mut self, data: Vec<u8>);
    fn set_uart06(&mut self, data: Vec<u8>);

    fn setup_uart03(&mut self, baut: u16);
    fn setup_uart04(&mut self, baut: u16);
    fn setup_uart05(&mut self, baut: u16);
    fn setup_uart06(&mut self, baut: u16);
}

pub trait AnalogNode {
    fn get_analog16_in01() -> u16;
    fn get_analog16_in02() -> u16;
    fn get_analog16_in03() -> u16;
    fn get_analog16_in04() -> u16;
    fn get_analog16_in05() -> u16;
    fn get_analog16_out() -> u16;
    fn set_analog16_out(val: u16) -> u16;
    fn get_temperature_in01() -> u16;
    fn get_temperature_in02() -> u16;
    fn get_temperature_in03() -> u16;
    fn reag_uart01() -> Vec<u8>;
    fn read_uart02() -> Vec<u8>;
    fn write_uart01() -> Vec<u8>;
    fn write_uart02() -> Vec<u8>;
}




