
use serde_derive::{Deserialize,Serialize};
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock, Mutex, RwLockReadGuard, RwLockWriteGuard};
use std::fmt::{self,Display};
use super::error::CanError;
use super::cob::{
    Address,
    CobData
};
use super::{
    can::Can,
    CAN,
    analog::AnalogNode,
    digital::{
        DigitalNode,
    },
    doppelmotor::{
        DoppelmotorNode,
    },
    analogext::AnalogextNode,
};
// use std::str::Bytes;

// use std::thread;
// use std::os::unix::net::{UnixStream, UnixListener};
// use std::pin::Pin;
// use futures::channel::mpsc::{channel, Sender};

// use std::thread;
// lazy_static! {

//     static ref CANBUS: Mutex<CanBus> = {
//         let can = Can::open("can0").unwrap();
//         Mutex::new(CanBus::new(can))
//     };
// }



pub enum Nodes
{

    pub analog1:AnalogNode,
    pub dm1: DoppelmotorNode,
    pub dm2: DoppelmotorNode,
    pub digital1: DigitalNode,
    pub digital2: DigitalNode,
    pub digital3: DigitalNode,
    pub analogext: AnalogextNode,
}

impl Nodes {
    pub fn new() -> Nodes {
        Nodes {
            analog1: AnalogNode::new(0x2),
            dm1: DoppelmotorNode::new(0x12),
            dm2: DoppelmotorNode::new(0x14),
            digital1: DigitalNode::new(0x18),
            digital2: DigitalNode::new(0x19),
            digital3: DigitalNode::new(0x1a),
            analogext: AnalogextNode::new(0x1c),
        }
    }
}


pub struct Bus {
    pub nodes: RwLock<Nodes>,
    pub data: HashMap<Address,CobData>
}

// pub struct CanMessage {
//     pub can : Arc<Mutex<Can>>,
//     pub addres : Address,
//     msg: Message,
// }

impl Bus {
    pub fn new() -> Bus {
        Bus {
            nodes: RwLock::new(Nodes::new()),
            data:  HashMap::new(),
        }
    }

    pub fn read(&mut self, address: Address) -> Result<CobData,CanError> {
        let rx = address.new_message(Vec::new())?;

        let tx = CAN::read(rx)?;
        let data = CobData::from(tx.get_data());
        self.data.insert(address,data.clone());
        Ok(data)
    }
    pub fn write(&mut self, address: Address,data: CobData ) -> Result<(),CanError> {
        let rx = address.new_message(data.clone().into())?;
        let _ = CAN::write(rx)?;
        self.data.insert(address,data.clone());
        Ok(())
    }
}



// pub async fn read_address(address:Address) -> Result<Message,CanError> {
    // Ok()
// }

// static mut CAN0 : Pin<&mut Can> = Pin::new(&mut Can::open("can0"));

// pub trait CanMsg: Default {
//     const ID:u32 = 0x2;
//     const INDEX:u16 = 0x2002;
//     const SUB:u8 = 0x2;
//     type Value : String;
//     type Error :CanError;

//     async fn read(&mut self) -> Result<Self::Value,Self::Error> {
//         let tx = Message::new_message(self.node,0x6101,0x3,Vec::new());
//         let rx = read(tx)?;
//         self.in03 = rx.to_u16();
//         Ok(self.in03)
//     }
// }

