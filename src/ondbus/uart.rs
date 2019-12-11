// use dbus::blocking::Connection;
use dbus::ffidisp::Connection;
use embedded_hal::serial::{Write,Read};
use dbus::Message;
// use std::time::Duration;
use crate::CanError;
// use bitvec::prelude::*;


pub struct Uart<'a> {
    conn: &'a Connection,
    node: String,
    iface: String,
    num: u8,
}

pub fn bitrate_num(bitratae: u32) -> u32 {
    match bitratae{
        9600   => 0,
        1200   => 1,
        2400   => 2,
        4800   => 3,
	    9600   => 4,
	    19200  => 5,
	    38400  => 6,
	    57600  => 7,
        115200 => 8,
        _ => 4,
    }
}

// "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart1"
// "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart2"
// "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetUart1"
// "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetUart2"
// "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "SetBautratet"

impl<'a> Uart<'a> {
    /// Create new Dingital node 
    /// 
    pub fn new(conn:&Connection,node:String,iface:String, num:u8) ->  Uart {
       Uart{conn,node,iface,num}
    }
    pub fn set_bitrate(&self,bitrate:u32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(),self.iface.as_str(), format!("SetBautrate{}",self.num).as_str()).unwrap().append1(bitrate_num(bitrate)),2000)?;
        Ok(())
    }
    pub fn read_string(&mut self) -> Result<String,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(),self.iface.as_str(), format!("GetUart{}",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap_or(String::from("")))
    }
    pub fn write_string(&mut self,value:String) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(),self.iface.as_str(), format!("SetUart{}",self.num).as_str()).unwrap().append1(value),2000)?;
        Ok(())
    }
}

/// Read half of a serial interface
///
impl<'a> Read<String> for Uart<'a>  {
    /// Read error
    type Error = CanError;

    /// Reads a single word from the serial interface
    fn read(&mut self) -> nb::Result<String, Self::Error> {
        let val = self.read_string()?;
        Ok(val)
    }
}
/// Write half of a serial interface
///
impl<'a> Write<String> for Uart<'a>  {
    /// Write error
    type Error = CanError;

    /// Writes a single word to the serial interface
    fn write(&mut self, word: String) -> nb::Result<(), Self::Error> {
        self.write_string(word)?;
        Ok(())
    }
    /// Ensures that none of the previously written words are still buffered
    fn flush(&mut self) -> nb::Result<(), Self::Error>{
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use dbus::ffidisp::Connection;
    use dbus::Message;
    use crate::CanError;

    #[test]
    fn test_uart() {
        // tcp:host=192.168.66.59,port=6666
        let c = Connection::open_private("tcp:host=192.168.66.59,port=6666").expect("open private connection tcp:host=192.168.66.59,port=6666");
        let analog_interface = "com.lar.nodes.Analog1";
        let dm_interface = "com.lar.nodes.Doppelmotor3";
        c.register();
        let mut auart1 = Uart::new(&c,"/com/lar/nodes/Analog1".to_owned(),analog_interface.to_owned(),1);
        let mut auart2 = Uart::new(&c,"/com/lar/nodes/Analog1".to_owned(),analog_interface.to_owned(),2);
        let mut uart11 = Uart::new(&c,"/com/lar/nodes/Doppelmotor1".to_owned(),dm_interface.to_owned(),1);
        let mut uart12 = Uart::new(&c,"/com/lar/nodes/Doppelmotor1".to_owned(),dm_interface.to_owned(),2);



        // let adc2 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn2".to_owned());
        // let adc3 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn3".to_owned());
        // let adc4 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn4".to_owned());
        // let adc5 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn5".to_owned());

        println!("AUART1 {}",auart1.read().unwrap());
        println!("AUART1 {}",auart1.read().unwrap());
        println!("UART1 {}",uart11.read().unwrap());
        println!("UART2 {}",uart12.read().unwrap());

        // println!("dout {}",dout1.is_high().unwrap());
        // dout1.set_high().unwrap();
        // assert!(dout1.is_high().unwrap(),"dout1 is not high");
        // dout1.set_low().unwrap();
        // assert!(dout1.is_low().unwrap(),"dout1 is not low");
        // let ad2val = adc2.read().unwrap();
        // let ad3val = adc3.read().unwrap();
        // let ad4val = adc4.read().unwrap();
        // let ad5val = adc5.read().unwrap();
        // println!("AIN:{}-{}-{}-{}-{}",ad1val,ad2val,ad3val,ad4val,ad5val);

    }
}

