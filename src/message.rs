use serde::{Serialize,Deserialize};
use std::fmt;
use std::convert::TryInto;
use crate::CanError;
use std::str::FromStr;

use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    // character::complete::{char, one_of},
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
    pub fn eds_index(&self) -> String {
        format!("{:X}sub{:X}",self.index,self.sub)
    }
}
/// Cmp function
impl Ord for Addr{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.node.cmp(&other.node)
    }
}
impl std::fmt::Display for Addr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:04}:{:02}", self.node,self.index,self.sub)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
pub struct Data {
    pub bytes: Vec<u8>,
    pub len : u8,
}
impl Data {
    pub fn to_u32(&self) -> u32 {
        match self.bytes.len() {
            1 => self.bytes[0] as u32,
            2 => (self.bytes[0] as u16 | (self.bytes[1] as u16) << 8) as u32,
            3 => (self.bytes[0] as u32|((self.bytes[1] as u32) << 8)|((self.bytes[2] as u32) << 16))as u32,
            4 => (self.bytes[0] as u32 | ((self.bytes[1] as u32) << 8) |  ((self.bytes[2] as u32) << 16) |((self.bytes[3] as u32) << 24)) as u32,
            _ => 0 as u32,
        }
    }
    pub fn to_string(&self) -> String {
        match self.len {
            0x21 => String::from_utf8(self.bytes.clone()).unwrap_or(String::from("")),
            _    => format!("{}",self.to_u32())
        }
    }
    pub fn from_value(&mut self,value:String) {
        match self.len {
            0x2F => {
                if let Ok(val) = u8::from_str(value.as_str()) {
                    self.bytes = val.to_le_bytes().to_vec();
                }
            },
            0x2D => {
                if let Ok(val) = u16::from_str(value.as_str()) {
                    self.bytes = val.to_le_bytes().to_vec();
                }
            },
            0x23 => {
                if let Ok(val) = u32::from_str(value.as_str()) {
                    self.bytes = val.to_le_bytes().to_vec();
                }
            },
            _ => {
                self.bytes = value.into_bytes();
            }
        }
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
            len: 0x2D,
        }
    }
}

impl From<u32> for Data {
    fn from(byte: u32) -> Self {
        let bytes:Vec<u8> = byte.to_le_bytes().to_vec();
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
pub struct Message {
    pub addr: Addr,
    pub data: Data,
}

pub struct Msg<T>
where
    T:From<Data> + Into<Data>,
   {
    pub addr: Addr,
    pub value: T,
}


impl Message {
    pub fn is_write(&self) -> bool {
        (self.data.len > 0 && self.data.bytes.len()>0)
    }
    pub fn set_value(&mut self,value:String) {
        self.data.from_value(value);
    }
    pub fn from_str(input:&str) -> IResult<&str,Message> {
        let (input,addr) = to_addr(input)?;
        let data = if input.len() > 2 {
            let (dt,v) = input.split_at(2);
            match dt.as_bytes()[1] {
                b'b' => Data::from(v.parse::<u8>().unwrap_or(0)),
                b'd' => Data::from(v.parse::<u16>().unwrap_or(0)),
                b't' => Data::from(v.parse::<u32>().unwrap_or(0)),
                b's' => Data::from(Vec::from(v.as_bytes())),
                _ => Data{bytes:Vec::new(),len:0},
            }
        }else {
            Data{
                bytes:Vec::new(),
                len:0
            }
        };
        Ok((input,Message{ addr, data }))
    }
    pub fn help() -> &'static str {
        HELP
    }
}

fn to_addr(input: &str) -> IResult<&str, Addr> {
    let (input, _) = tag("#")(input)?;
    let (input, (node, index, sub)) = tuple((nodeid, index, subindex))(input)?;
    Ok((input,Addr { node, index, sub }))
}

pub trait Datagramm {
    fn read(&self,addr: &Addr) -> Result<Data,CanError>;
    fn write(&self,addr: &Addr,tx:&Data) -> Result<(),CanError>;
}


impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}[ {}]",self.len, self.bytes.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + " "))
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}",self.addr, self.data)
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