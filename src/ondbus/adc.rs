// use dbus::blocking::Connection;
use dbus::ffidisp::Connection;
use embedded_hal::adc::Channel;
use dbus::Message;
// use super::uart::Uart;
// use std::time::Duration;
use crate::CanError;
// use bitvec::prelude::*;


struct Adc1;
pub struct ADC12<'a> {
    conn: &'a Connection,
    node: String,
    method: String,
    // msg:  Message,
}

impl<'a> ADC12<'a> {
    /// Create new Dingital node 
    /// Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn1").unwrap();
    pub fn new( conn:&Connection,node:String, method:String ) -> ADC12{
        ADC12{conn,node,method}
    }
    /// Read AD16bit value
    /// 
    pub fn read(&self) -> Result<u16,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can",self.node.as_str(), "com.lar.nodes.Analog1", self.method.as_str()).unwrap(),2000)?;
        let ain:u16 = r.get1().unwrap_or(0);
        Ok(ain)
    }
}

pub struct Temperatur01;
pub struct Temperatur02;
pub struct Temperatur03;

pub struct Temp<'a> {
    conn: &'a Connection,
    node: String,
    method: String,
}

impl<'a> Temp<'a> {
    /// Create new Dingital node 
    /// Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "Temperatur01").unwrap();
    pub fn new( conn:&Connection,node:String, method:String ) -> Temp{
        Temp{conn,node,method}
    }
    /// Read AD16bit value
    /// 
    pub fn read(&self) -> Result<u32,CanError> {
        let r = self.conn.send_with_reply_and_block(Message::new_method_call( "com.lar.service.can",self.node.as_str(), "com.lar.nodes.Analog1", self.method.as_str()).unwrap(),2000)?;
        let t:u32 = r.get1().unwrap_or(0);
        Ok(t)
    }
}

impl<'a> Channel<Adc1> for ADC12<'a> {
    type ID = u8;
    fn channel() -> u8 {5u8 }
}




#[cfg(test)]
mod tests {
    use super::*;
    use dbus::ffidisp::Connection;
    use dbus::Message;
    use crate::CanError;

    #[test]
    fn test_adc12() {
        // tcp:host=192.168.66.59,port=6666
        let c = Connection::open_private("tcp:host=192.168.66.59,port=6666").expect("open private connection tcp:host=192.168.66.59,port=6666");
        c.register();
        // let get_in2 = Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn2").unwrap();
        // let get_in3 = Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn3").unwrap();
        // let get_in4 = Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn4").unwrap();
        // let get_in5 = Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn5").unwrap();
        let adc1 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn1".to_owned());
        let adc2 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn2".to_owned());
        let adc3 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn3".to_owned());
        let adc4 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn4".to_owned());
        let adc5 = ADC12::new(&c,"/com/lar/nodes/Analog1".to_owned(),"GetIn5".to_owned());

        let ad1val = adc1.read().unwrap();
        println!("test{}",ad1val);
        // let ad2val = adc2.read().unwrap();
        // let ad3val = adc3.read().unwrap();
        // let ad4val = adc4.read().unwrap();
        // let ad5val = adc5.read().unwrap();
        // println!("AIN:{}-{}-{}-{}-{}",ad1val,ad2val,ad3val,ad4val,ad5val);

    }
}

