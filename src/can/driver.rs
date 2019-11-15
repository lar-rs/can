use crate::error::CanError;
use socketcan::*;
use std::fmt;
use std::io::{self};
use log::warn;
// use lazy_static::lazy_static;

// use bitvec::prelude::*;
// use nb::{self,block};
// use std::fmt;
// #[cfg(feature = "flame_it")]
// use flame;
// #[cfg(feature = "flame_it")]
// use flamer::*;

// use std::str::FromStr;
// use serde_derive::{Deserialize, Serialize};
// use serde::ser::{Serialize, SerializeStruct, Serializer};

// use bincode::{deserialize, serialize};



// lazy_static! {

    // static ref port:Port = Port::new();
    // static ref can0_result:Mutex<CanDevice> = {
        // match CANSocket::open("can0");
    //    Mutex::new(CANSocket::open("can0"))
    // };
    // let socket= ;
    // let can = Can::new(socket);
    // Mutex::new(can)
    // };
// }
//

// #[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
// pub struct CanAddr {
    // pub node: u32,
    // pub index: u16,
    // pub sub: u8,
// }
// impl CanAddr {
//     pub fn new( node:u32, index: u16, sub: u8) -> Self {
//         Self {
//             node: node,
//             index: index,
//             sub: sub,
//         }
//     }
// }

// #[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
// pub struct CanIndex {
//     pub addr: CanAddr,
//     pub data: Vec<u8>,
// }

// #[derive(Debug, Clone)]
// pub struct PCan {
//     innen: Arc<Mutex<CANSocket>>,
// }


// impl CanIndex{
//     pub fn new(node:u32,index:u16,sub:u8)->Self {
//         Self {
//             addr: CanAddr::new(node,index,sub),
//             data: Vec::new(),
//         }
//     }
// }

// impl PCan {
//     pub fn open(iface: &'static str) -> Result<PCan,CanError> {
//         let socket = CANSocket::open(iface)?;
//         let can = PCan {
//             innen: Arc::new(Mutex::new(socket)),
//         };
//         Ok(can)
//     }
// }

/// A TMCM module will respond with a `Reply` after receiving a `Command`.
#[derive(Debug)]
pub struct Message {
    node:  u32,
    frame: CANFrame,
    data:  Vec<u8>,
    // send: Option<CANFrame>,
}



// use mut_guard::*;

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:X}:{:?}", &self.node, self.data)
    }
}

impl Message {
    pub fn read_message(id: u32, index: u16, sub: u8) -> Message {
        let node = ((id & 0x7F) | 0x600) as u32;
        let mut d = [0 as u8; 8];
        d[0] = 0x40;
        d[1] = (index & 0xFF) as u8;
        d[2] = (index >> 8) as u8;
        d[3] = sub;
        let frame = CANFrame::new(node, &d, false, false).unwrap();
        let data = Vec::new();
        Message { node, frame, data }
    }
    pub fn to_u8(&self) -> u8 {
        if self.data.len() < 1 {
            warn!("message len({}) converted to u8",self.data.len());
            return 0;
        }
        self.data[0]
    }
    pub fn to_u16(&self) -> u16 {
        if self.data.len() < 2 {
            warn!("message len({}) converted to u16",self.data.len());
            return 0;
        }
        self.data[0] as u16 | ((self.data[1] as u16) << 8) as u16
    }
    pub fn to_i16(&self) -> i16 {
        if self.data.len() < 2 {
            return 0;
        }
        (self.data[0] as u16 | ((self.data[1] as u16) << 8)) as i16
    }
     pub fn to_u32(&self) -> u32 {
        if self.data.len() < 4 {
            return 0;
        }
        (self.data[0] as u32 | ((self.data[1] as u32) << 8) |  ((self.data[2] as u32) << 16) |((self.data[3] as u32) << 24)) as u32
    }
    pub fn to_i32(&self) -> i32 {
        if self.data.len() < 4 {
            return 0;
        }
        (self.data[0] as u32 | ((self.data[1] as u32) << 8) |  ((self.data[2] as u32) << 16) |((self.data[3] as u32) << 24)) as i32
    }
    pub fn get_data (&self ) -> Vec<u8>{
        self.data.clone()
    }
    pub fn new_message(id: u32, index: u16, sub: u8, data: Vec<u8>) -> Result<Message,CanError> {
        let node = ((id & 0x7F) | 0x600) as u32;
        let mut d = [0 as u8; 8];
        match data.len() {
            0 => {
                d[0] = 0x40;
            }
            1 => {
                d[0] = 0x2F; //101111
                d[4] = data[0];
            }
            2 => {
                d[0] = 0x2B;
                d[4] = data[0];
                d[5] = data[1];
            }
            3 => {
                d[0] = 0x23;
                d[4] = data[0];
                d[5] = data[1];
                d[6] = data[2];
            }
            4 => {
                d[0] = 0x23;
                d[4] = data[0];
                d[5] = data[1];
                d[6] = data[2];
                d[7] = data[3];
            }
            x => {
                d[0] = 0x21;
                d[4] = (x & 0xff) as u8;
                d[5] = (x >> 8) as u8;
            }
        }

        d[1] = (index & 0xFF) as u8;
        d[2] = (index >> 8) as u8;
        d[3] = sub;

        let frame = CANFrame::new(node, &d, false, false)?;
        // cm . flags    = 0;
        // cm . cob      = 0;
        // cm . id       = (node_index_nodeid(index) & 0x7F) + 0x600;
        // cm . length   = 8;
        // cm . data [1] = (char)(node_index_index(index) & 0xff);
        // cm . data [2] = (char)(node_index_index(index) >> 8);
        // cm . data [3] = (char)(node_index_subindex(index));
        // cm . data [4] = (char)(value & 0xff);
        // cm . data [5] = (char)(value >> 8);
        // cm . data [6] = (char)(value >> 16);
        // cm . data [7] = (char)(value >> 24);
        Ok(Message { node, frame, data })
    }
    pub fn new_u8 (id: u32, index: u16, sub: u8 ,value:u8) -> Result<Message,CanError> {
        let data = vec![ value ];
        Self::new_message(id, index, sub, data)
    }
    pub fn new_i16 (id: u32, index: u16, sub: u8 ,value:i16) -> Result<Message,CanError> {
        let data = vec![ (value & 0xff)as u8, (value >> 8)  as u8 ];
        Self::new_message(id, index, sub, data)
    }
    pub fn new_u16 (id: u32, index: u16, sub: u8 ,value:u16) -> Result<Message,CanError> {
        let data = vec![ (value & 0xff)as u8, (value >> 8)  as u8 ];
        Self::new_message(id, index, sub, data)
    }

    pub fn new_u32 (id: u32, index: u16, sub: u8 ,value:u32) -> Result<Message,CanError> {
        let data = vec![
        (value & 0xff)as u8,
        (value >> 8)  as u8,
        (value >> 16) as u8,
        (value >> 24) as u8,
        ];
        Self::new_message(id, index, sub, data)
    }
    pub fn new_i32 (id: u32, index: u16, sub: u8 ,value:i32) -> Result<Message,CanError> {
        let data = vec![
        (value & 0xff)as u8,
        (value >> 8)  as u8,
        (value >> 16) as u8,
        (value >> 24) as u8,
        ];
        Self::new_message(id, index, sub, data)
    }

    /// New long message for examle string
    pub fn new_long(id: u32, index: u16, sub: u8, data: Vec<u8>) -> Result<Message,CanError> {
        let node = ((id & 0x7F) | 0x600) as u32;
        let mut d = [0 as u8; 8];
        d[0] = 0x21;
        d[4] = (data.len() & 0xff) as u8;
        d[5] = (data.len() >> 8) as u8;
        d[1] = (index & 0xFF) as u8;
        d[2] = (index >> 8) as u8;
        d[3] = sub;
        let frame = CANFrame::new(node, &d, false, false)?;
        // cm . flags    = 0;
        // cm . cob      = 0;
        // cm . id       = (node_index_nodeid(index) & 0x7F) + 0x600;
        // cm . length   = 8;
        // cm . data [1] = (char)(node_index_index(index) & 0xff);
        // cm . data [2] = (char)(node_index_index(index) >> 8);
        // cm . data [3] = (char)(node_index_subindex(index));
        // cm . data [4] = (char)(value & 0xff);
        // cm . data [5] = (char)(value >> 8);
        // cm . data [6] = (char)(value >> 16);
        // cm . data [7] = (char)(value >> 24);
        Ok(Message { node, frame, data })
    }
    pub fn data_frame(&mut self, frame: CANFrame) {
        for index in 4..8 {
            self.data.push(frame.data()[index]);
        }
    }
    pub fn long_frame(&mut self, frame: CANFrame) -> u8 {
        // self.data
        let lang = frame.data()[0] as u8;
        match lang & 0xE {
            0x0 => {
                for index in 1..8 {
                    self.data.push(frame.data()[index]);
                }
            }
            0x02 => {
                for index in 1..7 {
                    self.data.push(frame.data()[index]);
                }
            }
            0x04 => { //
                for index in 1..6 {
                    self.data.push(frame.data()[index]);
                }
            }
            0x06 => { //
                for index in 1..5 {
                    self.data.push(frame.data()[index]);
                }
            }
            0x08 => {
                for index in 1..4 {
                    self.data.push(frame.data()[index]);
                }
            }
            0x0A => {
                for index in 1..3 {
                    self.data.push(frame.data()[index]);
                }
            }
            0x0C => {
                for index in 1..2 {
                    self.data.push(frame.data()[index]);
                }
            }
            _ => {
                self.data.push(frame.data()[1]);
            }
        };
        lang & 0x1
    }
    pub fn to_string(&self) -> String {
        String::from_utf8(self.data.clone()).unwrap()
    }

}

