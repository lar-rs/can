// use dbus::blocking::Connection;
use dbus::ffidisp::Connection;
use embedded_hal::digital::v2::{InputPin,OutputPin};
use dbus::Message;
// use std::time::Duration;
use crate::CanError;
// use bitvec::prelude::*;


pub struct DigIN<'a> {
    conn: &'a Connection,
    rxmsg: Message,
}


impl<'a> DigIN<'a> {
    /// Create new Dingital node 
    /// for example Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(self.input) 
    pub fn new(conn:&Connection,rxmsg:Message) -> DigIN{
        DigIN{conn,rxmsg}
    }
}


impl<'a> InputPin for DigIN<'a> {
    type Error = CanError;
    /// Is the input pin high?
    fn is_high(&self) -> Result<bool,CanError> {
        let r = self.conn.send_with_reply_and_block(self.rxmsg.method_return(),2000)?;
        let digin = bool::from(r.get1().unwrap_or(false));
        Ok(digin)
    }

    /// Is the input pin low?
    fn is_low(&self) -> Result<bool,CanError> {
        let r = self.conn.send_with_reply_and_block(self.rxmsg.method_return(),2000)?;
        let digin = bool::from(r.get1().unwrap_or(false));
        Ok(!digin)
    }
}  

pub struct DigOUT<'a> {
    conn:  &'a Connection,
    rxmsg: Message,
    low:   Message,
    high:  Message,
}

impl<'a> DigOUT<'a> {
    /// Create new Dingital node  
    /// 
    /// for example Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append2(self.output,false)
    /// for example Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append2(self.output,true)
    /// for example Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "GetDigitalOut").unwrap().append1(self.output)
    pub fn new(conn:&Connection,rxmsg:Message,low:Message,high: Message) -> DigOUT {
        DigOUT{conn,rxmsg,low,high}
    }
}


/// Single digital push-pull output pin
impl<'a> OutputPin for DigOUT<'a> {
    type Error = CanError;
    /// Error type

    /// Drives the pin low
    ///
    fn set_low(&mut self) -> Result<(), Self::Error> {
        let _ = self.conn.send_with_reply_and_block(self.low.method_return(),2000)?;
        Ok(())
    }

    /// Drives the pin high
    ///
    fn set_high(&mut self) -> Result<(), Self::Error> {
        let _ = self.conn.send_with_reply_and_block(self.high.method_return(),2000)?;
        Ok(())
    }
}

impl<'a> InputPin for DigOUT<'a> {
    type Error = CanError;
    /// Is the input pin high?
    fn is_high(&self) -> Result<bool,CanError> {
        let r = self.conn.send_with_reply_and_block(self.rxmsg.method_return(),2000)?;
        let digin = bool::from(r.get1().unwrap_or(false));
        Ok(digin)
    }

    /// Is the input pin low?
    fn is_low(&self) -> Result<bool,CanError> {
        let high = self.is_high()?;
        Ok(high)
    }
}