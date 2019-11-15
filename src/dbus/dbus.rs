use super::error::CanError;
use dbus;
use dbus::Error as DBusError;
use dbus::{BusType, Connection, Message};
// use once_cell::sync::OnceCell;
// use serde_json::Value;
// use std::num::ParseIntError;
// use std::str::FromStr;
use futures::prelude::*;
// use super::mio::CanBus;

use std::{
    sync::{Arc,Mutex},
//     pin::Pin,
};

// pub struct MethodNode {
//     node: Node,
//     m: Message,
// }



pub struct CanDBus{
    conn : Connection,
}



impl CanDBus {
    pub fn message(&mut self, m:Message) -> Message {
        self.conn.send_with_reply_and_block(m, 2000).unwrap()
    }
    pub fn new(bus: BusType) -> Result<CanDBus, DBusError> {
        let c = Connection::get_private(bus)?;
        Ok(Self {
            conn: c,
        })
    }
}


// <method name="GetIndexValue">
// 			<arg direction="in" type="s" name="index"/>
// 			<arg direction="out" type="s" name="value"/>
// 			<arg direction="out" type="s" name="value_type"/>
// 		</method>
// 		<!-- SetIndexValue:
// 			@index:    Index.
// 		   	@value:    Intex value in string format.
// 		   	@is_done:  Done if value changed.
// 		    @since: 1.10 Set intex value-->
// 		<method name="SetIndexValue">
// 			<arg direction="in" type="s" name="index"/>
// 			<arg direction="in" type="s" name="value"/>
// 			<arg direction="out" type="b" name="is_done"/>
// 		</method>
impl CanDBus {
   fn get_ain01(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn1").unwrap());
        r.get1().unwrap()
    }
    fn get_ain02(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn2").unwrap());
        r.get1().unwrap()
    }
    fn get_ain03(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn3").unwrap());
        r.get1().unwrap()
    }
    fn get_ain04(&mut self) -> u16 {
       let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn4").unwrap());
       r.get1().unwrap()
    }
    fn get_ain05(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn5").unwrap());
        r.get1().unwrap()
    }
    fn get_aout(&mut self) -> u16{
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetOut").unwrap());
        r.get1().unwrap()
    }
    fn set_aout(&mut self, val: u16) {
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "SetOut").unwrap().append1(val));
    }
    fn get_temp01(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp1").unwrap());
        r.get1().unwrap()
    }
    fn get_temp02(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp1").unwrap());
        r.get1().unwrap()
    }
     fn get_temp03(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp3").unwrap());
        r.get1().unwrap()
    }
    ///com.lar.nodes.Digital16
    fn get_dig18in(&mut self,num:u8) -> bool {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(num));
        bool::from(r.get1().unwrap())
    }
    fn get_dig18out(&mut self,num:u8) -> bool  {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "GetDigitalOut").unwrap().append1(num));
        bool::from(r.get1().unwrap())
    }
    fn set_dig18out(&mut self,num:u8,val:bool) {
        // let outdig = self.get_dig18out();
       self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append1(num).append1(val));
    }
     ///com.lar.nodes.Digital16
    fn get_dig19in(&mut self,num:u8) -> bool {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(num));
        bool::from(r.get1().unwrap())
    }
    fn get_dig19out(&mut self,num:u8) -> bool  {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "GetDigitalOut").unwrap().append1(num));
        bool::from(r.get1().unwrap())
    }
    fn set_dig19out(&mut self,num:u8,val:bool) {
        // let outdig = self.get_dig18out();
       self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append1(num).append1(val));
    }
    ///com.lar.nodes.Digital16
    fn get_uart01(&mut self) -> Vec<u8>  {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart1").unwrap());
        r.get1().unwrap()
    }
    fn get_uart02(&mut self) -> Vec<u8> {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart2").unwrap());
        r.get1().unwrap()
    }
    fn get_uart03(&mut self) -> Vec<u8> {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
        r.get1().unwrap()
    }
    fn get_uart04(&mut self) -> Vec<u8> {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
        r.get1().unwrap()
    }
    fn get_uart05(&mut self) -> Vec<u8> {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
        r.get1().unwrap()
    }
    fn get_uart06(&mut self) -> Vec<u8> {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
        r.get1().unwrap()
    }

    fn set_uart01(&mut self, data: Vec<u8>) {
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
    }
    fn set_uart02(&mut self, data: Vec<u8>) {
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
    }
    fn set_uart03(&mut self, data: Vec<u8>) {
       self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
    }
    fn set_uart04(&mut self, data: Vec<u8>) {
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
    }
    fn set_uart05(&mut self, data: Vec<u8>) {
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
    }
    fn set_uart06(&mut self, data: Vec<u8>) {
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
    }

    fn setup_uart03(&mut self, baut: u16){
       self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
    }
    fn setup_uart04(&mut self, baut: u16){
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
    }
    fn setup_uart05(&mut self, baut: u16){
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
    }
    fn setup_uart06(&mut self, baut: u16){
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
    }
    // fn dm11_setup(speed:u8,max:u16,invert:bool);
    // fn dm11_move(sped:u8,max:u16);
    // fn dm11_setup(speed:u8,max:u16);
}



// impl RpcMio for CanDBus {
// 	fn protocol_version(&self) -> Result<String> {
// 		Ok("version1".into())
// 	}

// 	fn read_index(&self,node:u32,index:u16, sub: u8) -> Result<String,CanError> {

//     }
//     fn write_index(&self,node:u32,index:u16, sub: u8,data:Vec<u8>) -> Result<(),CanError>;

//     #[rpc(name = "get_ain01")]
//     fn get_ain01(&self) -> Result<u16, CanError>;
//     #[rpc(name = "get_ain02")]
//     fn get_ain02(&self) -> Result<u16, CanError>;
//      #[rpc(name = "get_ain03")]
//     fn get_ain03(&self) -> Result<u16, CanError>;
//      #[rpc(name = "get_ain04")]
//     fn get_ain04(&self) -> Result<u16, CanError>;
//      #[rpc(name = "get_ain05")]
//     fn get_ain05(&self) -> Result<u16, CanError>;
//      #[rpc(name = "get_aout")]
//     fn get_aout(&self, num: u8) -> Result<u16, CanError>;
//     #[rpc(name = "set_aout")]
//     fn set_aout(&self, num: u8, val: u16) -> FutureResult<(), CanError>;
//     #[rpc(name = "get_temp01")]
//     fn get_temp01(&self,num:u8) -> Result<u16,CanError>;
//     #[rpc(name = "get_temp02")]
//     fn get_temp01(&self,num:u8) -> Result<u16,CanError>;
//     #[rpc(name = "get_temp03")]
//     fn get_temp01(&self,num:u8) -> Result<u16,CanError>;


//     #[rpc(name = "get_din")]
//     fn get_din(&self,digit:u8) ->Result<bool,CanError>;
//     #[rpc(name = "get_dout")]
//     fn get_dout(&self,digit:u8) ->Result<bool,CanError>;
//      #[rpc(name = "set_dout")]
//     fn set_dout(&self,digit:u8,value:bool) ->Result<(),CanError>;

//     #[rpc(name = "setup_uart")]
//     fn setup_uart(&self,uart:u8,baut: u16)->Result<(),Error>;
//     #[rpc(name = "read_uart")]
//     fn read_uart(&self,uart:u8)->FutureResult<Vec<u8>,Error>;
//     #[rpc(name = "write_uart")]
//     fn write_uart(&self,uart:u8,data: Vec<u8>)->Result<(),Error>;
// }
