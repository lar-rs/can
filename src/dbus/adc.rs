// use dbus::blocking::Connection;
use dbus::ffidisp::Connection;
// use embedded_hal::digital::v2::{InputPin,OutputPin};
use dbus::Message;
// use std::time::Duration;
use crate::CanError;
// use bitvec::prelude::*;



pub struct ADC12<'a> {
    conn: &'a Connection,
    msg:  Message,
}

impl<'a> ADC12<'a> {
    /// Create new Dingital node 
    /// Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn1").unwrap();
    pub fn new( conn:&Connection, msg:Message ) -> ADC12{
        ADC12{conn,msg}
    }
    /// Read AD16bit value
    /// 
    pub fn read(&self) -> Result<u16,CanError> {
        let r = self.conn.send_with_reply_and_block(self.msg.method_return(),2000)?;
        let ain:u16 = r.get1().unwrap_or(0);
        Ok(ain)
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
        let get_in1 = Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn1").unwrap();
        let get_in2 = Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn2").unwrap();
        let get_in3 = Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn3").unwrap();
        let get_in4 = Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn4").unwrap();
        let get_in5 = Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn5").unwrap();
        let adc1 = ADC12::new(&c,get_in1);
        let adc2 = ADC12::new(&c,get_in2);
        let adc3 = ADC12::new(&c,get_in3);
        let adc4 = ADC12::new(&c,get_in4);
        let adc5 = ADC12::new(&c,get_in5);

    }
}

