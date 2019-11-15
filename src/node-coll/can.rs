/// Can data type
/// 0x0001= 	1 - 1 bit boolean
/// 0x0002= 	1 - 8 bit signed integer
/// 0x0003=     1 - 16 bit signed integer
/// 0x0004=     1 - 32 bit signed integer
/// 0x0005=     1 - 8 bit unsigned integer
/// 0x0006= 	1 - 16 bit unsigned integer
/// 0x0007= 	1 - 32 bit unsigned integer
/// 0x0008=	    1 - single precision floating point
/// 0x0009= 	1 - visible string
/// 0x000A= 	1 - octet string  z.Z wird nicht unterstürtzt.
/// 0x000B= 	0 - date
/// 0x000C= 	0 - time of day
/// 0x000D= 	0 - time difference
/// 0x000E= 	0 - bit string
/// 0x000F= 	0 - domain
/// 0x0020= 	1 - PDO CommPar
/// 0x0021= 	1 - PDO mapping
/// 0x0022= 	1 - SDO parameter

/// Ablauf all --> MSG.Send(addres) Message = resive_antwort()
/// #![allow(unused_variables)]

// use crate::error::CanError;


// use bitvec::prelude::*;
use socketcan;
pub use socketcan::{CANFrame,ConstructionError,CANSocket};
use std::fmt;
use std::io::{self};
// use nb::{self,block};
// use std::fmt;
#[cfg(feature = "flame_it")]
use flame;
#[cfg(feature = "flame_it")]
use flamer::*;
use log::warn;

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
use std::sync::atomic::{AtomicPtr};




pub struct  Can{
    socket: CANSocket,
}




impl Can {
    pub fn new(socket:CANSocket) -> Self {
        // socket.set_nonblocking(true)?;
        Can { socket: socket}
    }


    pub fn read(&self, node:u32,index:u16,sub: u8) -> Result<Vec<u8>,CanError> {
        let mut store = [0 as u8; 8];
        let node = ((node & 0b0111_1111) | 0b0011_0000_0000) as u32;
        store[0] = 0b0100_0000u8;
        store[1] = index as u8 & 0b1111_1111u8;
        store[2] = (index >> 8) as u8;
        store[3] = sub;
        // let mut data_bits = BitSlice::<LittleEndian, u8>::from_slice_mut(&mut store);
        // data_bits.set(1,true);
        let mut data   = Vec::new();
        let  rx   = CANFrame::new(node, &store, false, false)?;
        self.socket.write_frame(&rx)?;
        let rx_frame = self.socket.read_frame()?;
        let mut len  =  rx_frame.data()[5];
        if  rx_frame.data()[0] == 0x41 {
            // Long type reading
            store[0] = 0b0110_0000u8;
            while len > 7  {
                self.socket.write_frame(&CANFrame::new(node, &store, false, false)?)?;
                let rf = self.socket.read_frame()?;
                let te = (rf.data()[0] & 0xE)>>1;
                data.extend_from_slice(&rf.data()[1 ..te as usize]);
                len -= te;
                store[0] ^= 0b0001_0000u8;
            }
        }
        Ok(data)
    }
    pub fn write (&self, node: u32,index:u16,sub: u8, data: &[u8]) -> Result< Vec<u8>, CanError> {
        let mut store = [0 as u8; 8];
        let node = ((node & 0b0111_1111) | 0b0011_0000_0000) as u32;
        // 0x19 0b0001_1001
        // 0x18 0b0001_1000
        // 0b0111_111 = 0x7F) | 0b0011_0000_0000 = 0x600) as u32;
        let tx_data = Vec::new();
        match data.len() {
            0 => {
                store[0] = 0x40;
            }
            1 => {
                store[0] = 0x2F; //101111
                store[4] = data[0];
            }
            2 => {
                store[0] = 0x2B;
                store[4] = data[0];
                store[5] = data[1];
            }
            3 => {
                store[0] = 0x23;
                store[4] = data[0];
                store[5] = data[1];
                store[6] = data[2];
            }
            4 => {
                store[0] = 0x23;
                store[4] = data[0];
                store[5] = data[1];
                store[6] = data[2];
                store[7] = data[3];
            }
            x => {
                store[0] = 0x21;
                store[4] = (x & 0xff) as u8;
                store[5] = (x >> 8) as u8;
            }
        }
        store[1] = index as u8 & 0b1111_1111u8;
        store[2] = (index >> 8) as u8;
        store[3] = sub;
        self.socket.write_frame(&CANFrame::new(node, &store, false, false)?)?;

        let rx_frame = self.socket.read_frame()?;
        let te = rx_frame.data()[0] & 0b0110_0000u8;
        match te {
            0b0010_0001 => { // read
                store[0] = 0b0110_0000u8;
                while len > 7  {
                    self.socket.write_frame(&CANFrame::new(node, &store, false, false)?)?;
                    let rf = self.socket.read_frame()?;
                    let te = (rf.data()[0] & 0xE)>>1;
                    data.extend_from_slice(&rf.data()[1 ..te as usize]);
                    len -= te;
                    store[0] ^= 0b0001_0000u8;
            }
            },
            0b0110_0000 => { // write
                let pos:usize =0;
                let mut toggle:u8 = 0x00;
                for chunk in data.chunks(7) {
                    let l = 7 - chunk.len() as u8;
                    let mut d = vec![(toggle+l << 1)];
                    d.extend_from_slice(chunk);
                    self.socket.write_frame(&CANFrame::new(node, &d, false, false)?)?;
                    let _ = self.socket.read_frame()?;
                    toggle ^= 0b0001_0000u8;
                }
            }
            _ => {

            }
        }

        Ok(tx_data)
    }
}







//

// lazy_static! {
//     static ref CAN0: CANSocket = CANSocket::open("can0");
// }

// pub struct Address{
//     node: u32,
//     index: u16,
//     sub: u8,
// }

// impl Address {
//     pub new(node:u32,index:u16,sub:u8) -> Address
// }






#[cfg(test)]
mod tests {
    use super::*;
    /// use socketcan::CANFrame;
    /// Attempt delivery of two messages, using a oneshot channel
    /// to prompt the second message in order to demonstrate that
    /// waiting for CAN reads is not blocking.
    // #[test]
    // fn test_board() {
    //     let board = Can::open("can0").unwrap();
    //     // board.send_message(Message::read_message(0x18,0x2002,0x0)).unwrap();
    //     let message = board
    //         .read_message(Message::read_message(0x18, 0x2002, 0x1))
    //         .map_err(|err| {
    //             println!("write io error: {:?}", err);
    //         })
    //         .unwrap();
    //     println!("Message:{:?}", message.to_string());
    //     board
    //         .write_message(Message::new_message(0x18, 0x6300, 0x1, vec![0xff, 0x00]))
    //         .map_err(|err| {
    //             println!("write io error: {:?}", err);
    //         })
    //         .unwrap();
    //     let d = "FRENZEL+BERG".to_owned().as_bytes().to_vec();
    //     let message = block!(board.write_message(Message::new_long(0x18, 0x2002, 0x1, d.clone())))
    //         .map_err(|err| {
    //             println!("write io error: {:?}", err);
    //         })
    //         .unwrap();
    //     println!("Message:{:?}", message);

    //     // let message = board.read_message(Message::read_message(0x18,0x2002,0x2)).unwrap();
    //     // println!("Message:{:?}",message.to_string());
    //     // let meboard.read_message(Message::read_message(0x18,0x2002,0x2)).unwrap();
    //     // board.send_message(Message::read_message(0x18,0x2002,0x3)).unwrap();
    //     // board.send_message(Message::read_message(0x18,0x2002,0x4)).unwrap();
    //     // board.send_message(Message::read_message(0x18,0x6000,0x1)).unwrap();
    //     // board.send_message(Message::read_message(0x18,0x6100,0x1)).unwrap();
    //     // board.send_message(Message::read_message(0x18,0x6300,0x1)).unwrap();
    //     // board.send_message(Message::read_message(0x18,0x6320,0x1)).unwrap();
    //     // board.send_message(Message::read_message(0x18,0x6320,0x1)).unwrap();
    //     // board.write_dout(0).unwrap();
    //     // assert_eq!(board.read_din().unwrap(),0);
    //     // assert_eq!(board.read_dout().unwrap(),0);
    //     // board.write_dout(0b1111100u16).unwrap();
    //     // assert_eq!(board.read_dout().unwrap(),0b1111100u16);
    // }
    #[test]
    fn test_bits () {

        // let first_byte = BitVec::<BigEndian, u8>::from_element(0xE);
        // let mut store: u8  = 0xE as u8;
        // let bs = store.as_bitslice::<LittleEndian>();
        // let drain_type  = BitVec::<BigEndian, u8>::from_element(0xE).drain(4 .. 7).collect::<BitVec>();
        // println!("{}->{}",&bs,&bs[4..7]);
        // let src = 0b0100_1011u8;
// let bits = src.as_bitslice::<BigEndian>();
// let mut windows = bits.windows(4);
// assert_eq!(windows.next(), Some(&bits[0 .. 4]));
// assert_eq!(windows.next(), Some(&bits[1 .. 5]));
// assert_eq!(windows.next(), Some(&bits[2 .. 6]));
// assert_eq!(windows.next(), Some(&bits[3 .. 7]));
// assert_eq!(windows.next(), Some(&bits[4 .. 8]));
// assert!(windows.next().is_none());
    }
}

// use lazy_static::lazy_static!{
// static ref CAN0: Shared<CAN> ={
// let can = CanBus::open();
// Shared::new(can)
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let mut rt = tokio::runtime::current_thread::Runtime::new().unwrap();
//         let handle = rt.handle();
//         tx.send(handle).unwrap();

//         runtime_raw::set_runtime(&TokioCurrentThread);
//         let forever = futures01::future::poll_fn(|| {
//             Ok::<futures01::Async<()>, ()>(futures01::Async::NotReady)
//         });
//         rt.block_on(forever).unwrap();
//     });

//     let handle = rx.recv().unwrap();
//     Mutex::new(handle)
// };
// }
// }
