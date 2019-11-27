// use dbus::blocking::Connection;
use dbus::ffidisp::Connection;
use embedded_hal::serial::{Write,Read};
use dbus::Message;
// use std::time::Duration;
use crate::CanError;
// use bitvec::prelude::*;


pub struct Uart<'a> {
    conn: &'a Connection,
    rxmsg: Message,
    txmsg: Message,
    bitrate: Message,
}



// "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart1"
// "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart2"
// "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetUart1"
// "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetUart2"
// "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "SetBautratet"

impl<'a> Uart<'a> {
    /// Create new Dingital node 
    /// 
    pub fn new(conn:&Connection,rxmsg: Message,txmsg:Message,bitrate:Message) ->Uart{
       Uart{conn,rxmsg,txmsg,bitrate}
    }
    pub fn set_bitrate(&self,bitrate:u32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(self.bitrate.method_return().append1(bitrate),2000);
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
        let r = self.conn.send_with_reply_and_block(self.rxmsg.method_return(),2000).unwrap();
        Ok(r.get1().unwrap_or(String::from("")))
    }
}
/// Write half of a serial interface
///
impl<'a> Write<String> for Uart<'a>  {
    /// Write error
    type Error = CanError;

    /// Writes a single word to the serial interface
    fn write(&mut self, word: String) -> nb::Result<(), Self::Error> {
        let _ = self.conn.send_with_reply_and_block(self.txmsg.method_return().append1(word),2000);
        Ok(())
    }

    /// Ensures that none of the previously written words are still buffered
    fn flush(&mut self) -> nb::Result<(), Self::Error>{
        Ok(())
    }
}

