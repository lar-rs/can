pub mod driver;
use socketcan::*;
use serde::{Serialize,Deserialize};
use crate::{
    error::CanError,
};
use async_std::io;
use bincode::{deserialize, serialize};
// use serde_json::from_slice;

#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
pub struct CanMsg {
    pub node: u32,
    pub index: u16,
    pub sub: u8,
    pub data:Vec<u8>,
}
pub struct  Can {
    socket: CANSocket,
}


impl Can {
    pub fn new(socket:CANSocket) -> Self {
        // socket.set_nonblocking(true)?;
        Can { socket: socket}
    }
    pub fn open(iface: &'static str) -> Result<Can,CanError> {
        let socket = CANSocket::open(iface)?;
        let can = Self::new(socket);
        Ok(can)
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
    pub fn write (&mut self, node: u32,index:u16,sub: u8, data: &[u8]) -> Result< Vec<u8>, CanError> {
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
                let mut len =rx_frame.data()[4];
                store[0] = 0b0110_0000u8;
                while len > 7  {
                    self.socket.write_frame(&CANFrame::new(node, &store, false, false)?)?;
                    let rf = self.socket.read_frame()?;
                    let te = (rf.data()[0] & 0xE)>>1;
                    // data.extend_from_slice(&rf.data()[1 ..te as usize]);
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
    pub async fn transmit(&mut self,msg : CanMsg)-> io::Result<Vec<u8>> { 
        if msg.data.len() > 0 {
           let data = self.write(msg.node,msg.index,msg.sub, msg.data.as_slice())?;
           return Ok(data);
        } else {
           self.read(msg.node,msg.index,msg.sub)?;
        };
        Ok(Vec::new())
    }
    pub async fn message(&mut self, data: Vec<u8>) -> io::Result<Vec<u8>> {
         
        let msg:CanMsg = match deserialize(&data) {
            Ok(msg) => msg,
            _ => serde_json::from_slice(&data)?,
        };
        self.transmit(msg).await
    }
}

