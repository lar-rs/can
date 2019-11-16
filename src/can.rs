pub mod driver;
use socketcan::*;
use serde::{Serialize,Deserialize};
use crate::{
    error::CanError,
};
use async_std::io;
// use bincode::{deserialize, serialize};
use std::collections::BTreeMap;
use std::cmp::{Ordering,Ord};

// use serde_json::from_slice;

#[derive(Serialize,Deserialize,PartialEq,PartialOrd,Eq,Debug,Clone)]
pub struct Addr {
    pub node: u32,
    pub index: u16,
    pub sub: u8,
}

impl Addr {
    pub fn new(node:u32,index:u16,sub:u8) -> Addr {
        Addr {
            node: node,
            index: index,
            sub: sub,
        }
    }
}
/// Cmp function
impl Ord for Addr{
    fn cmp(&self, other: &Self) -> Ordering {
            self.node.cmp(&other.node)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
pub struct Data {
    pub bytes: Vec<u8>,
}

impl From<Vec<u8>> for Data {
    fn from(bytes: Vec<u8>) -> Self {
        Data{
            bytes: bytes,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
pub struct TxMsg {
    pub addr: Addr,
    pub data: Data,
}
#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
pub struct RxMsg {
    pub addr: Addr,
}



pub struct  Can {
    socket: CANSocket,
    data:   BTreeMap<Addr,Data>,

}



impl Can {
    pub fn new(socket:CANSocket) -> Self {
        Can{ 
            socket: socket,
            data: BTreeMap::new(),
        }
    }
    pub fn open(iface: &'static str) -> Result<Can,CanError> {
        let socket = CANSocket::open(iface)?;
        let can = Self::new(socket);
        Ok(can)
    }

    pub fn read(&mut self, addr: Addr) -> Result<Vec<u8>,CanError> {
        let mut store = [0 as u8; 8];
        let node = ((addr.node & 0b0111_1111) | 0b0011_0000_0000) as u32;
        store[0] = 0b0100_0000u8;
        store[1] = addr.index as u8 & 0b1111_1111u8;
        store[2] = (addr.index >> 8) as u8;
        store[3] = addr.sub;
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
        self.data.insert(addr,data.clone().into());
        Ok(data)
    }
    pub fn write (&mut self,addr: Addr,tx:Data) -> Result<(),CanError> {
        let mut store = [0 as u8; 8];
        let node = ((addr.node & 0b0111_1111) | 0b0011_0000_0000) as u32;
        // 0x19 0b0001_1001
        // 0x18 0b0001_1000
        // 0b0111_111 = 0x7F) | 0b0011_0000_0000 = 0x600) as u32;
        // let tx_data = Vec::new();
        let data = tx.bytes.as_slice();
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
        store[1] = addr.index as u8 & 0b1111_1111u8;
        store[2] = (addr.index >> 8) as u8;
        store[3] = addr.sub;
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
        self.data.insert(addr,tx.clone());
        Ok(())
    }
    /// transmit message.
    pub async fn transmit(&mut self,msg:TxMsg)-> io::Result<()> { 
        self.write(msg.addr,msg.data)?;
        Ok(())
    }
    // recieve message.
    pub async fn recieve(&mut self,msg:RxMsg) -> io::Result<Vec<u8>> {
        let data = self.read(msg.addr)?;
        Ok(data)
    }
}

