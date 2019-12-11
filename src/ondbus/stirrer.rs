// use dbus::blocking::Connection;
use dbus::ffidisp::Connection;
// use embedded_hal::digital::v2::{InputPin,OutputPin};
use dbus::Message;
// use std::time::Duration;
use crate::CanError;
// use bitvec::prelude::*;


pub struct Stirrer<'a> {
    conn: &'a Connection,
    node: String,
    num: u8,
}


impl<'a> Stirrer<'a> {
    /// Create new Dingital node 
    /// for example Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(self.input) 
    pub fn new(conn:&Connection,node:String,num:u8) -> Stirrer{
        Stirrer{conn,node,num}
    }
    pub fn set_stepper_on(&mut self,value:bool) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetStepper{}On",self.num).as_str()).unwrap().append1(value),2000)?;
        Ok(())
    }
    pub fn get_stepper_on(&mut self) -> Result<bool,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetStepper{}On",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
    pub fn set_delay(&mut self,delay:u32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetStepper{}Delay",self.num).as_str()).unwrap().append1(delay),2000)?;
        Ok(())
    }
    pub fn get_delay(&mut self) -> Result<u32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetStepper{}Delay",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }

    pub fn set_current(&mut self,current:u32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetStepper{}Current",self.num).as_str()).unwrap().append1(current),2000)?;
        Ok(())
    }
    pub fn get_current(&mut self) -> Result<u32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetStepper{}Current",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use dbus::ffidisp::Connection;
    use dbus::Message;
    use crate::CanError;

    #[test]
    fn test_stirrer() {
        // tcp:host=192.168.66.59,port=6666
        let c = Connection::open_private("tcp:host=192.168.66.59,port=6666").expect("open private connection tcp:host=192.168.66.59,port=6666");
        c.register();
        let mut st1  = Stirrer::new(&c,"/com/lar/nodes/Doppelmotor1".to_owned(),2);

        println!("ST1:ON    {}",st1.get_stepper_on().unwrap());
        println!("ST1:DELAY {}",st1.get_delay().unwrap());
        println!("ST1:CURR  {}",st1.get_current().unwrap());

   }
}
