use dbus;
// use dbus::Error as DBusError;
use dbus::blocking::Connection;
use std::time::Duration;

use dbus::{Message};
// use crate::error::CanError;
// use once_cell::sync::OnceCell;
// use serde_json::Value;
// use std::num::ParseIntError;
// use std::str::FromStr;
// use futures::prelude::*;
// use super::mio::CanBus;

// use std::{
    // sync::{Arc,Mutex},
//     pin::Pin,
// };
// 
// pub struct MethodNode {
//     node: Node,
//     m: Message,
// }



pub struct CanDBus<'a>{ 
    conn : &'a Connection,
}

pub struct 

impl<'a> CanDBus<'a> {
    pub fn read(&self, m:Message) -> Message {
        self.conn.send_with_reply_and_block(m, 2000).unwrap()
    }
    pub fn new(conn: &Connection) -> Result<CanDBus<'a>, DBusError> {
        // let c = Connection::get_private(bus)?;
        Ok(Self {
            conn: &conn,
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
impl<'a> CanDBus<'a> {
//     /// get analog input 16bit
   fn get_ain01(&self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn1").unwrap());
        r.get1().unwrap()
    }
//     fn get_ain02(&self) -> u16 {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn2").unwrap());
//         r.get1().unwrap()
//     }
//     fn get_ain03(&self) -> u16 {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn3").unwrap());
//         r.get1().unwrap()
//     }
//     fn get_ain04(&self) -> u16 {
//        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn4").unwrap());
//        r.get1().unwrap()
//     }
//     fn get_ain05(&self) -> u16 {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn5").unwrap());
//         r.get1().unwrap()
//     }
//     fn get_aout(&self) -> u16{
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetOut").unwrap());
//         r.get1().unwrap()
//     }
//     fn set_aout(&self, val: u16) {
//         self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "SetOut").unwrap().append1(val));
//     }
//     fn get_temp01(&self) -> u16 {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp1").unwrap());
//         r.get1().unwrap()
//     }
//     fn get_temp02(&self) -> u16 {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp1").unwrap());
//         r.get1().unwrap()
//     }
//      fn get_temp03(&self) -> u16 {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp3").unwrap());
//         r.get1().unwrap()
//     }
//     ///com.lar.nodes.Digital16
//     fn get_dig18in(&self,num:u8) -> bool {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(num));
//         bool::from(r.get1().unwrap())
//     }
//     fn get_dig18out(&self,num:u8) -> bool  {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "GetDigitalOut").unwrap().append1(num));
//         bool::from(r.get1().unwrap())
//     }
//     fn set_dig18out(&self,num:u8,val:bool) {
//         // let outdig = self.get_dig18out();
//        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append1(num).append1(val));
//     }
//      ///com.lar.nodes.Digital16
//     fn get_dig19in(&self,num:u8) -> bool {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(num));
//         bool::from(r.get1().unwrap())
//     }
//     fn get_dig19out(&self,num:u8) -> bool  {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "GetDigitalOut").unwrap().append1(num));
//         bool::from(r.get1().unwrap())
//     }
//     fn set_dig19out(&self,num:u8,val:bool) {
//         // let outdig = self.get_dig18out();
//        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append1(num).append1(val));
//     }
//     ///com.lar.nodes.Digital16
//     fn get_uart01(&self) -> Vec<u8>  {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart1").unwrap());
//         r.get1().unwrap()
//     }
//     fn get_uart02(&self) -> Vec<u8> {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart2").unwrap());
//         r.get1().unwrap()
//     }
//     fn get_uart03(&self) -> Vec<u8> {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
//         r.get1().unwrap()
//     }
//     fn get_uart04(&self) -> Vec<u8> {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
//         r.get1().unwrap()
//     }
//     fn get_uart05(&self) -> Vec<u8> {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
//         r.get1().unwrap()
//     }
//     fn get_uart06(&self) -> Vec<u8> {
//         let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
//         r.get1().unwrap()
//     }

//     fn set_uart01(&self, data: Vec<u8>) {
//         self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
//     }
//     fn set_uart02(&self, data: Vec<u8>) {
//         self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
//     }
//     fn set_uart03(&self, data: Vec<u8>) {
//        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
//     }
//     fn set_uart04(&self, data: Vec<u8>) {
//         self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
//     }
//     fn set_uart05(&self, data: Vec<u8>) {
//         self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
//     }
//     fn set_uart06(&self, data: Vec<u8>) {
//         self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
//     }

//     fn setup_uart03(&self, baut: u16){
//        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
//     }
//     fn setup_uart04(&self, baut: u16){
//         self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
//     }
//     fn setup_uart05(&self, baut: u16){
//         self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
//     }
//     fn setup_uart06(&self, baut: u16){
//         self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
//     }
//     // fn dm11_setup(speed:u8,max:u16,invert:bool);
//     // fn dm11_move(sped:u8,max:u16);
//     // fn dm11_setup(speed:u8,max:u16);
}



// // impl RpcMio for CanDBus {
// // 	fn protocol_version(&self) -> Result<String> {
// // 		Ok("version1".into())
// // 	}

// // 	fn read_index(&self,node:u32,index:u16, sub: u8) -> Result<String,CanError> {

// //     }
// //     fn write_index(&self,node:u32,index:u16, sub: u8,data:Vec<u8>) -> Result<(),CanError>;

// //     #[rpc(name = "get_ain01")]
// //     fn get_ain01(&self) -> Result<u16, CanError>;
// //     #[rpc(name = "get_ain02")]
// //     fn get_ain02(&self) -> Result<u16, CanError>;
// //      #[rpc(name = "get_ain03")]
// //     fn get_ain03(&self) -> Result<u16, CanError>;
// //      #[rpc(name = "get_ain04")]
// //     fn get_ain04(&self) -> Result<u16, CanError>;
// //      #[rpc(name = "get_ain05")]
// //     fn get_ain05(&self) -> Result<u16, CanError>;
// //      #[rpc(name = "get_aout")]
// //     fn get_aout(&self, num: u8) -> Result<u16, CanError>;
// //     #[rpc(name = "set_aout")]
// //     fn set_aout(&self, num: u8, val: u16) -> FutureResult<(), CanError>;
// //     #[rpc(name = "get_temp01")]
// //     fn get_temp01(&self,num:u8) -> Result<u16,CanError>;
// //     #[rpc(name = "get_temp02")]
// //     fn get_temp01(&self,num:u8) -> Result<u16,CanError>;
// //     #[rpc(name = "get_temp03")]
// //     fn get_temp01(&self,num:u8) -> Result<u16,CanError>;


// //     #[rpc(name = "get_din")]
// //     fn get_din(&self,digit:u8) ->Result<bool,CanError>;
// //     #[rpc(name = "get_dout")]
// //     fn get_dout(&self,digit:u8) ->Result<bool,CanError>;
// //      #[rpc(name = "set_dout")]
// //     fn set_dout(&self,digit:u8,value:bool) ->Result<(),CanError>;

// //     #[rpc(name = "setup_uart")]
// //     fn setup_uart(&self,uart:u8,baut: u16)->Result<(),Error>;
// //     #[rpc(name = "read_uart")]
// //     fn read_uart(&self,uart:u8)->FutureResult<Vec<u8>,Error>;
// //     #[rpc(name = "write_uart")]
// //     fn write_uart(&self,uart:u8,data: Vec<u8>)->Result<(),Error>;
// // }
