// use dbus::blocking::Connection;
use dbus::ffidisp::Connection;
use dbus::Message;
// use std::time::Duration;
use crate::CanError;
// use bitvec::prelude::*;


pub struct GP<'a> {
    conn: &'a Connection,
    node: String,
    num: u8,
}


impl<'a> GP<'a> {
    /// Create new Dingital node 
    /// for example Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(self.input) 
    pub fn new(conn:&Connection,node:String,num:u8) ->GP{
        GP{conn,node,num}
    }
    pub fn set_pump(&mut self,value:bool) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetPump{}",self.num).as_str()).unwrap().append1(value),2000)?;
        Ok(())
    }
    pub fn get_pump(&mut self) -> Result<bool,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetPump{}",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
    pub fn set_mod(&mut self,value:i32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetPump{}Mode",self.num).as_str()).unwrap().append1(value),2000)?;
        Ok(())
    }
    pub fn get_mod(&mut self) -> Result<i32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetPump{}Mode",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
    pub fn set_left(&mut self,value:i32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetPump{}Left",self.num).as_str()).unwrap().append1(value),2000)?;
        Ok(())
    }
    pub fn get_left(&mut self) -> Result<i32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetPump{}Left",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
    pub fn set_speed(&mut self,value:i32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetPump{}Speed",self.num).as_str()).unwrap().append1(value),2000)?;
        Ok(())
    }
    pub fn get_speed(&mut self) -> Result<i32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetPump{}Speed",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
    pub fn set_interval_pulse(&mut self,value:i32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetPump{}IntervalPulse",self.num).as_str()).unwrap().append1(value),2000)?;
        Ok(())
    }
    pub fn get_interval_pulse(&mut self) -> Result<i32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetPump{}IntervalPulse",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
    pub fn set_interval_time(&mut self,value:i32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetPump{}IntervalTime",self.num).as_str()).unwrap().append1(value),2000)?;
        Ok(())
    }
    pub fn get_interval_time(&mut self) -> Result<i32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetPump{}IntervalTime",self.num).as_str()).unwrap(),2000)?;
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
        let mut gp1  =GP::new(&c,"/com/lar/nodes/Doppelmotor1".to_owned(),1);
        let mut gp2  =GP::new(&c,"/com/lar/nodes/Doppelmotor1".to_owned(),2);

        // 01GP1
        println!("GP1:ON    {}",gp1.get_pump().unwrap());
        println!("GP1:MODE  {}",gp1.get_mod().unwrap());
        println!("GP1:LEFT  {}",gp1.get_left().unwrap());
        println!("GP1:SPEED {}",gp1.get_speed().unwrap());
        println!("GP1:PULSE {}",gp1.get_interval_pulse().unwrap());
        println!("GP1:TIME  {}",gp1.get_interval_time().unwrap());

        // 01GP2
        println!("GP2:ON    {}",gp2.get_pump().unwrap());
        println!("GP2:MODE  {}",gp2.get_mod().unwrap());
        println!("GP2:LEFT  {}",gp2.get_left().unwrap());
        println!("GP2:SPEED {}",gp2.get_speed().unwrap());
        println!("GP2:PULSE {}",gp2.get_interval_pulse().unwrap());
        println!("GP2:TIME  {}",gp2.get_interval_time().unwrap());

   }
}

