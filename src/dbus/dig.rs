// use dbus::blocking::Connection;
use dbus::ffidisp::Connection;
use embedded_hal::digital::v2::{InputPin,OutputPin};
use dbus::Message;
// use std::time::Duration;
use crate::CanError;
// use bitvec::prelude::*;


pub struct DigIN<'a> {
    conn: &'a Connection,
    node: String,
    input: u32,
}


impl<'a> DigIN<'a> {
    /// Create new Dingital node
    /// for example Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(self.input)
    pub fn new(conn:&Connection,node:String,input:u32) -> DigIN{
        DigIN{conn,node,input}
    }
}


impl<'a> InputPin for DigIN<'a> {
    type Error = CanError;
    /// Is the input pin high?
    fn is_high(&self) -> Result<bool,CanError> {
        let r = self.conn.send_with_reply_and_block( Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(self.input) ,2000)?;
        let digin = bool::from(r.get1().unwrap_or(false));
        Ok(digin)
    }

    /// Is the input pin low?
    fn is_low(&self) -> Result<bool,CanError> {
        let digin = self.is_high()?;
        Ok(!digin)
    }
}

pub struct DigOUT<'a> {
    conn:  &'a Connection,
    node: String,
    output: u32,
}

impl<'a> DigOUT<'a> {
    /// Create new Dingital node
    ///
    /// for example
    /// for example Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append2(self.output,true)
    /// for example Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "GetDigitalOut").unwrap().append1(self.output)
    pub fn new(conn:&Connection,node:String,output:u32) -> DigOUT {
        DigOUT{conn,node,output}
    }
}


/// Single digital push-pull output pin
impl<'a> OutputPin for DigOUT<'a> {
    type Error = CanError;
    /// Error type

    /// Drives the pin low
    ///
    fn set_low(&mut self) -> Result<(), Self::Error> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append2(self.output,false),2000)?;
        Ok(())
    }

    /// Drives the pin high
    ///
    fn set_high(&mut self) -> Result<(), Self::Error> {
        let _ = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append2(self.output,true),2000)?;
        Ok(())
    }
}

impl<'a> InputPin for DigOUT<'a> {
    type Error = CanError;
    /// Is the input pin high?
    fn is_high(&self) -> Result<bool,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can", self.node.as_str(), "com.lar.nodes.Digital16", "GetDigitalOut").unwrap().append1(self.output),2000)?;
        let digin = bool::from(r.get1().unwrap_or(false));
        Ok(digin)
    }

    /// Is the input pin low?
    fn is_low(&self) -> Result<bool,CanError> {
        let high = self.is_high()?;
        Ok(!high)
    }
}


pub struct DigitalNode<'a> {
    pub node: u32,
    pub din: [DigIN<'a>;16],
    pub dout: [DigOUT<'a>;16],
}

impl <'a> DigitalNode<'a> {
    pub fn new(c:&Connection,node: u32) -> DigitalNode{

        let node_path = match node {
            0x18 => format!("/com/lar/nodes/Digital1"),
            0x19 => format!("/com/lar/nodes/Digital2"),
            _    => format!("/com/lar/nodes/Digital1"),
        };
        let din = [
            DigIN::new(c,node_path.clone(),1),
            DigIN::new(c,node_path.clone(),2),
            DigIN::new(c,node_path.clone(),3),
            DigIN::new(c,node_path.clone(),4),
            DigIN::new(c,node_path.clone(),5),
            DigIN::new(c,node_path.clone(),6),
            DigIN::new(c,node_path.clone(),7),
            DigIN::new(c,node_path.clone(),8),
            DigIN::new(c,node_path.clone(),9),
            DigIN::new(c,node_path.clone(),10),
            DigIN::new(c,node_path.clone(),11),
            DigIN::new(c,node_path.clone(),12),
            DigIN::new(c,node_path.clone(),13),
            DigIN::new(c,node_path.clone(),14),
            DigIN::new(c,node_path.clone(),15),
            DigIN::new(c,node_path.clone(),16),
        ];
        let dout = [
            DigOUT::new(c,node_path.clone(),1),
            DigOUT::new(c,node_path.clone(),2),
            DigOUT::new(c,node_path.clone(),3),
            DigOUT::new(c,node_path.clone(),4),
            DigOUT::new(c,node_path.clone(),5),
            DigOUT::new(c,node_path.clone(),6),
            DigOUT::new(c,node_path.clone(),7),
            DigOUT::new(c,node_path.clone(),8),
            DigOUT::new(c,node_path.clone(),9),
            DigOUT::new(c,node_path.clone(),10),
            DigOUT::new(c,node_path.clone(),11),
            DigOUT::new(c,node_path.clone(),12),
            DigOUT::new(c,node_path.clone(),13),
            DigOUT::new(c,node_path.clone(),14),
            DigOUT::new(c,node_path.clone(),15),
            DigOUT::new(c,node_path.clone(),16),
        ];
        DigitalNode { node , din, dout}
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use dbus::ffidisp::Connection;
    use dbus::Message;
    use crate::CanError;

    #[test]
    fn test_adc12() {
        /// tcp:host=192.168.66.59,port=6666
        let c = Connection::open_private("tcp:host=192.168.66.59,port=6666").expect("open private connection tcp:host=192.168.66.59,port=6666");
        c.register();
        let digital1 = String::from("/com/lar/nodes/Digital1");

        let mut dout1 = DigOUT::new(&c,digital1,1);

        let din1  = DigIN::new(&c,"/com/lar/nodes/Digital1".to_owned(),1);


        // let adc2 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn2".to_owned());
        // let adc3 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn3".to_owned());
        // let adc4 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn4".to_owned());
        // let adc5 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn5".to_owned());

        println!("din {}",din1.is_high().unwrap());
        println!("dout {}",dout1.is_high().unwrap());
        dout1.set_high().unwrap();
        assert!(dout1.is_high().unwrap(),"dout1 is not high");
        dout1.set_low().unwrap();
        assert!(dout1.is_low().unwrap(),"dout1 is not low");
        // let ad2val = adc2.read().unwrap();
        // let ad3val = adc3.read().unwrap();
        // let ad4val = adc4.read().unwrap();
        // let ad5val = adc5.read().unwrap();
        // println!("AIN:{}-{}-{}-{}-{}",ad1val,ad2val,ad3val,ad4val,ad5val);

    }
}

