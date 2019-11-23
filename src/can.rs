use socketcan::*;
use serde::{Serialize,Deserialize};
use crate::{
    error::{CanError,wrong_data},
};
// use async_std::io;
use std::fmt;
// use bincode::{deserialize, serialize};
use std::collections::BTreeMap;
use std::cmp::{Ordering,Ord};
use lazy_static::lazy_static;

use std::convert::TryInto;
// use std::iter::Iterator;
// use serde_json::from_slice;
use std::time::Duration;
use std::sync::{Mutex,Arc,RwLock};
use async_std::io::{BufReader,BufWriter};
use async_std::io;

use nom::{
    IResult,
    named,
    alt,
    bytes::complete::{escaped, tag, take_while_m_n},
    character::complete::{alphanumeric1 as alphanumeric, char, one_of},
    combinator::map_res,
    sequence::tuple};

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
impl fmt::Display for Addr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:04}:{:02}", self.node,self.index,self.sub)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
pub struct Data {
    pub bytes: Vec<u8>,
    pub len : u8,
}
impl Data {
    fn data<T:From<Data>>(self) -> T{
        self.into()
    }
}
impl From<Vec<u8>> for Data {
    fn from(bytes: Vec<u8>) -> Self {
        Data{
            bytes: bytes,
            len: 0x21,
        }
    }
}
impl From<u8> for Data {
    fn from(byte: u8) -> Self {
        Data{
            bytes: vec![byte],
            len: 0x2F,
        }
    }
}

impl From<u16> for Data {
    fn from(byte: u16) -> Self {
        Data{
            bytes: byte.to_le_bytes().to_vec(),
            len: 0x23,
        }
    }
}

impl From<u32> for Data {
    fn from(byte: u32) -> Self {
        let mut bytes:Vec<u8> = byte.to_le_bytes().to_vec();
        Data{
            bytes:bytes,
            len: 0x23,
        }
    }
}
impl Into<u8> for Data {
    fn into(self) -> u8 {
        let (int_bytes, _) = self.bytes.as_slice().split_at(std::mem::size_of::<u8>());
        u8::from_le_bytes(int_bytes.try_into().unwrap())
    }
}

impl Into<u16> for Data {
    fn into(self) -> u16 {
        let (int_bytes, _) = self.bytes.as_slice().split_at(std::mem::size_of::<u16>());
        u16::from_le_bytes(int_bytes.try_into().unwrap())
    }
}
impl Into<u32> for Data {
    fn into(self) -> u32 {
        let (int_bytes, _) = self.bytes.as_slice().split_at(std::mem::size_of::<u32>());
        u32::from_le_bytes(int_bytes.try_into().unwrap())
    }
}
impl Into<Vec<u8>> for Data {
    fn into(self) -> Vec<u8> {
        self.bytes.clone()
    }
}

fn id_hex(input: &str) -> Result<u32, std::num::ParseIntError> {
  u32::from_str_radix(input, 32)
}
fn index_hex(input: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(input, 16)
}

fn sub_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 8)
}
  
fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

fn nodeid(input: &str) -> IResult<&str, u32> {
  map_res(
    take_while_m_n(1, 2, is_hex_digit),
    id_hex
  )(input)
}
fn index(input: &str) -> IResult<&str, u16> {
    map_res(
      take_while_m_n(4, 4, is_hex_digit),
      index_hex
    )(input)
}
fn subindex(input:&str) -> IResult<&str, u8> {
    map_res(
      take_while_m_n(1, 2, is_hex_digit),
      sub_hex
    )(input)
} 

#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
pub struct CanMsg {
    pub addr: Addr,
    pub data: Option<Data>,
}


pub fn to_canmsg(input: &str) -> IResult<&str, CanMsg> {
    let (input, _) = tag("#")(input)?;
    let (input, (node, index, sub)) = tuple((nodeid, index, subindex))(input)?;
    let addr =Addr { node, index, sub };
    let mut data: Option<Data> = None;
    if input.len() > 2 {
        let (dt,v) = input.split_at(2);
        match dt.as_bytes()[1] {
            b's' => data = Some(Data::from(Vec::from(v.as_bytes()))),
            b'b' => data = Some(Data::from(v.parse::<u8>().unwrap_or(0))),
            b'd' => data = Some(Data::from(v.parse::<u16>().unwrap_or(0))),
            b't' => data = Some(Data::from(v.parse::<u32>().unwrap_or(0))),
            _ => data = None,
        }
    };
    Ok((input,CanMsg{ addr, data }))
}
// pub fn can_data(input: &str) -> IResult<&str,Data> {
    
    // Ok(addr,None)
// }



impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}[ {}]",self.len, self.bytes.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + " "))
    }
}

impl fmt::Display for CanMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            Some(data) => write!(f, "{} {}",self.addr, data),
            None => write!(f, "{} None",self.addr),
        }
    }
}
// impl From<Vec<u8>> for Data {
    // fn from(bytes: Vec<u8>) -> Self {
        // Data{
            // bytes: bytes,
            // len: 0x21,
        // }
    // }
// }


lazy_static! {
    static ref DATA: RwLock<BTreeMap<Addr, Data>> = RwLock::new(BTreeMap::new());
}


#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
pub struct RxMsg {
    pub addr: Addr,
}

pub fn incomming ( addr:Addr, data:Data ) {
    let mut btree = DATA.write().unwrap();
    btree.insert(addr,data);
}


pub trait Datagram {
    fn read(&self,addr: &Addr) -> Result<Data,CanError>;
    fn write(&self,addr: &Addr,tx:&Data) -> Result<(),CanError>;
}

// pub trait AsyncDatagram {
    // fn read(&self,addr: &Addr) -> Result<Data,CanError>;
    // fn write(&self,addr: &Addr,tx:&Data) -> Result<(),CanError>;
// }

pub struct  Can {
    socket: CANSocket,
}

impl Can {
    pub fn new(socket:CANSocket) -> Self {
        Can{ 
            socket: socket,
        }
    }
    pub fn open(iface: &str) -> Result<Can,CanError> {
        let socket = CANSocket::open(iface)?;
        socket.set_read_timeout(Duration::from_secs(1))?;
        socket.set_write_timeout(Duration::from_secs(1))?;
        let can = Self::new(socket);
        Ok(can)
    }
    fn read_data(&self, addr: &Addr) -> Result<Data,CanError> {
        let mut store = [0 as u8; 8];
        let node = ((addr.node & 0b0111_1111) | 0b0011_0000_0000) as u32;
        store[0] = 0b0100_0000u8;
        store[1] = addr.index as u8 & 0b1111_1111u8;
        store[2] = (addr.index >> 8) as u8;
        store[3] = addr.sub;
        let mut data:Vec<u8>   = Vec::new();
        let  rx   = CANFrame::new(node, &store, false, false)?;
        self.socket.write_frame(&rx)?;
        let rx_frame = self.socket.read_frame()?;
        let mut len  =  rx_frame.data()[5];
        if  rx_frame.data()[0] == 0x41 {
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
        if data.len() < 4 {
           data.resize(4, 0);
        }
        Ok(data.into())
    }
    fn write_data(&self,addr: &Addr,tx:&Data) -> Result<(),CanError> {
        let mut store = [0 as u8; 8];
        let node = ((addr.node & 0b0111_1111) | 0b0011_0000_0000) as u32;
        // 0x19 0b0001_1001
        // 0x18 0b0001_1000
        // 0b0111_111 = 0x7F) | 0b0011_0000_0000 = 0x600) as u32;
        // let tx_data = Vec::new();
        let data = tx.bytes.as_slice();
        store[0] = tx.len;
        match tx.len {
            0x2F => {  //0010 1111
                if data.len()> 0 {
                    store[4] = data[0];
                }
            }
            0x2B => { //0010 1011
                for (i, &item) in tx.bytes.iter().enumerate() {
                    if i < 2 {
                        store[4+i] = item;
                    }
                }
            }
            0x23 => { //0010 0011
                for (i, &item) in tx.bytes.iter().enumerate() {
                    if i < 4 {
                        store[4+i] = item;
                    }
                }
            }
            0x21 => {
                store[4] = (data.len() & 0xff) as u8;
                store[5] = (data.len() >> 8) as u8;
            }
            _ =>  {
                return Err(wrong_data(format!("Data type {} can't write ",tx.len)));
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
                // let pos:usize =0;
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
        Ok(())
    }
    pub fn processing(&self,msg:&CanMsg) -> Result<Data,CanError> {
        match &msg.data {
            Some(data) => { self.write_data(&msg.addr, &data)?; Ok(data.clone()) },
            None => Ok(self.read_data(&msg.addr)?),
        }
    }
}


pub async fn send_message(can:&Can,msg:&CanMsg) -> io::Result<Data>  {
    match &msg.data {
        Some(data) => { 
            can.write(&msg.addr,&data)?;
            Ok(data.clone())
        },
        None => Ok(can.read(&msg.addr)?),
    }
}

impl Datagram for Can {
    fn read(&self,addr:&Addr) -> Result<Data,CanError>  {
        let data = self.read_data(addr)?;
        Ok(data)
    }
    fn write(&self ,addr:&Addr,data:&Data) -> Result<(),CanError> {
        self.write_data(&addr,&data)?;
        Ok(())
    }
}

pub struct SharedCan {
    can : Arc<Mutex<Can>>,
    data: Arc<RwLock<BTreeMap<Addr, Data>>>,
}

impl SharedCan {
    pub fn open (iface: &str) ->  Result<SharedCan,CanError>  {
        let can  = Arc::new(Mutex::new(Can::open(iface)?));
        let data: Arc<RwLock<BTreeMap<Addr, Data>>> = Arc::new(RwLock::new(BTreeMap::new()));
        Ok(SharedCan{can,data})
    } 
 
    fn read_data(&mut self,addr:&Addr) -> Result<Data,CanError> {
        let mut can = self.can.lock().unwrap();
        let data = can.read(addr)?;
        Ok(data)
    }
    fn write_data(&mut self ,addr:&Addr,data:&Data) -> Result<(),CanError> {
        let mut can = self.can.lock().unwrap();
        can.write(addr,data)?;
        Ok(())
    }
    pub fn update(&mut self,addr:&Addr,data:&Data) -> Result<(),CanError> {
        let mut wdata = self.data.write().unwrap();
        wdata.insert(addr.clone(),data.clone());
        Ok(())
    }
    pub fn read(&mut self,addr:&Addr) -> Result<Data,CanError>  {
        let data = self.read_data(addr)?;
        self.update(addr,&data)?;
        Ok(data)
    }
    pub fn write(&mut self ,addr:&Addr,data:&Data) -> Result<(),CanError> {
        self.write_data(&addr,&data)?;
        self.update(addr,data)?;
        Ok(())
    }
   
}





// pub trait CanRw{ 
//     const IFACE: String;

//     fn read(&self, addr: Addr) -> Result<Data,CanError>{
//         read_data(Self::IFACE,addr)
//     }
//     fn write(&self, addr: Addr,tx:Data) -> Result<(),CanError>  {
//         write_data(Self::IFACE,addr,tx)
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_addr() {
        assert_eq!(can_addr("#02600001"), Ok(("", Addr {
            node: 0x2,
            index: 0x6000,
            sub: 1,
          })));
        assert_eq!(can_addr("#0260001"), Ok(("", Addr {
            node: 0x2,
            index: 0x6000,
            sub: 1,
          })));
        assert_eq!(can_addr("#0260001"), Ok(("", Addr {
            node: 0x2,
            index: 0x6000,
            sub: 1,
          })));
    }
}

pub const HELP: &'static str = r#"
Format #Address Data
#NodeIndexSub
Data:
    s - String
    b - u8
    d - u16
    t - u32
Examples:
   `#12600001 sLong data` - Write long data 
       12   - Node id 0x12
       6000 - Index 
       01   - subindex
       s    - String data format

   `#12600001` read long data
   `#18610101 d4534
"#;