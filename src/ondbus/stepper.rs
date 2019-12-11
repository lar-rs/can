// use dbus::blocking::Connection;
use dbus::ffidisp::Connection;
// use embedded_hal::digital::v2::{InputPin,OutputPin};
use dbus::Message;
// use std::time::Duration;
use crate::CanError;
// use bitvec::prelude::*;

/// Stepper 
/// Axis X,Y,Z
pub struct Stepper<'a> {
    conn: &'a Connection,
    node: String,
    num: u8,
}


impl<'a> Stepper<'a> {
    /// Create new Stepper 
    /// for example Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(self.input) 
    pub fn new(conn:&Connection,node:String,num:u8) -> Stepper {
        Stepper{conn,node,num}
    }
    pub fn set_stepper_mod(&mut self,mode:u32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetStepper{}Mode",self.num).as_str()).unwrap().append1(mode),2000)?;
        Ok(())
    }
    pub fn get_stepper_mod(&mut self) -> Result<u32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetStepper{}Mode",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
    pub fn goto_position(&mut self,pos:u32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("setStepper{}GoPos",self.num).as_str()).unwrap().append1(pos),2000)?;
        Ok(())
    }
    pub fn get_position(&mut self) -> Result<u32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetStepper{}GoPos",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
    pub fn set_command(&mut self,command:u32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetStepper{}CommandStatus",self.num).as_str()).unwrap().append1(command),2000)?;
        Ok(())
    }
    pub fn set_max(&mut self,max:u32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetStepper{}MaxPos",self.num).as_str()).unwrap().append1(max),2000)?;
        Ok(())
    }
    pub fn get_max(&mut self) -> Result<u32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetStepper{}MaxPos",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
    pub fn set_parameter(&mut self,parameter:u32) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetStepper{}Parameter",self.num).as_str()).unwrap().append1(parameter),2000)?;
        Ok(())
    }
    pub fn get_parameter(&mut self) -> Result<u32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetStepper{}Parameter",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
    pub fn set_endschalter_invert(&mut self,invert:bool) -> Result<(),CanError> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("SetStepper{}EndschalterInvert",self.num).as_str()).unwrap().append1(invert),2000)?;
        Ok(())
    }
    pub fn final_pos(&mut self) -> Result<bool,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetStepper{}FinalPosition",self.num).as_str()).unwrap(),2000)?;
        Ok(r.get1().unwrap())
    }
    pub fn pos_old(&mut self) -> Result<u32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Doppelmotor3", format!("GetStepper{}PosOld",self.num).as_str()).unwrap(),2000)?;
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
    fn test_stepper() {
        // tcp:host=192.168.66.59,port=6666
        let c = Connection::open_private("tcp:host=192.168.66.59,port=6666").expect("open private connection tcp:host=192.168.66.59,port=6666");
        c.register();
        let mut x  = Stepper::new(&c,"/com/lar/nodes/Doppelmotor1".to_owned(),1);
        let mut y  = Stepper::new(&c,"/com/lar/nodes/Doppelmotor2".to_owned(),1);
        let mut z  = Stepper::new(&c,"/com/lar/nodes/Doppelmotor2".to_owned(),2);


        // let adc2 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn2".to_owned());
        // let adc3 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn3".to_owned());
        // let adc4 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn4".to_owned());
        // let adc5 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn5".to_owned());

        println!("X:max   {}",x.get_max().unwrap());
        println!("X:pos   {}",x.get_position().unwrap());
        println!("X:par   {}",x.get_parameter().unwrap());
        println!("X:final {}",x.final_pos().unwrap());

        println!("Y:max   {}",y.get_max().unwrap());
        println!("Y:pos   {}",y.get_position().unwrap());
        println!("Y:par   {}",y.get_parameter().unwrap());
        println!("Y:final {}",y.final_pos().unwrap());

        println!("Z:max   {}",z.get_max().unwrap());
        println!("Z:pos   {}",z.get_position().unwrap());
        println!("Z:par   {}",z.get_parameter().unwrap());
        println!("Z:final {}",z.final_pos().unwrap());

   }
}

