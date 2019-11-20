


// use futures::prelude::*;
// use serde_derive::{Deserialize,Serialize};
use lazy_static::lazy_static;
// use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
// use futures::prelude::*;
// use futures_timer::Delay;

lazy_static! {

}
// use super::can0::CAN;
// use super::can::{
    // Message,
//  };

use super::error::CanError;

pub type AInput16 = u16;
/// AnalogNode
// #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalogNode {

    pub node:   u32,
    pub in01:   u16,
    pub in02:   u16,
    pub in03:   u16,
    pub in04:   u16,
    /// AnalogIn 6111:1
    pub in05:   u16,
    /// AnalogOut 6120:1
    pub out01:  u16,
    pub uart01: Vec<u8>,
    pub uart02: Vec<u8>,
    pub temp01: u16,
    pub temp02: u16,
    pub temp03: u16,
}


impl Default for AnalogNode {
    fn default() -> Self{
        Self {
            node: 0x2,
            in01: 3262,
            in02: 3000,
            in03: 1500,
            in04: 1000,
            in05: 0,
            out01: 0,
            uart01: Vec::new(),
            uart02: Vec::new(),
            temp01: 774,
            temp02: 774,
            temp03: 774,
         }
    }
}





impl AnalogNode {
    pub fn new(id:u32,can:) -> Self{
        Self {
            node:   id,
            in01:   3262,
            in02:   3000,
            in03:   1500,
            in04:   1000,
            in05:   0,
            out01:  0,
            uart01: Vec::new(),
            uart02: Vec::new(),
            temp01: 774,
            temp02: 774,
            temp03: 774,
         }
    }
    pub fn state(&mut self) -> AnalogNode {
        self.clone()
    }
}



pub async fn info() -> String {
    "Analog info"
}

pub async fn get_input01(node:AnalogNode) -> u16{
    2000
}
pub async fn get_input02(node:AnalogNode) -> u16{
    2000
}
pub async fn get_input03(node:AnalogNode) -> u16{
    2000
}
pub async fn get_input04(node:AnalogNode) -> u16{
    2000
}
pub async fn get_input05(node:AnalogNode) -> u16{
    2000
}
pub async fn get_out(node:AnalogNode) -> u16 {
    1500
} 
pub async fn get_uart01(node:AnalogNode) -> Vec<u8> {
    Vec::new()
}
pub async fn set_uart01(node:AnalogNode,data:Vec<u8>) -> Vec<u8> {
    node.uart01 = data.clone()
}
pub async fn setup_uart01(node:AnalogNode,bautrate:u8) {
    node.bautrate = bautrate;
}

pub async fn get_uart02(node:AnalogNode) -> Vec<u8> {
    Vec::new()
}
pub async fn set_uart02(node:AnalogNode,data:Vec<u8>) -> Vec<u8> {
    node.uart02 = data.clone()
}
pub async fn setup_uart02(node:AnalogNode,bautrate:u8) {
    node.bautrate = bautrate;
}




