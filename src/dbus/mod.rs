pub mod dig;
pub mod uart;
pub mod adc;
pub mod stepper;
pub mod stirrer;


// use async_std::sync::channel;
use crossbeam::channel::{Sender,Receiver};
// use async_std::task;
use crossbeam::channel as channel;
use dbus::Message as DBusMessage;
use crate::CanError;
// use serde::{Serialize,Deserialize};
use crate::message::{Message};

// A router
pub struct Route {
    // Send `(ipc_recv, send)` to this router to add a route
    // from the IPC receiver to the sender.
    sender:   Sender<Message>,
    resiver:  Receiver<Message>,

}



use dbus::ffidisp::Connection;

// let async brocker()

// Lazily initialize a singleton router,
// so we only end up with one routing thread per process.
// lazy_static! {

    // static ref ROUTE: Route = {

    // };
// }
//    CanBus
//    "/etc/candaemon/eds/AnalogExt.eds"               | NodeAnalogExtObject |  /com/lar/nodes/Analogext1        0x1c
//    "/etc/candaemon/eds/Digital.eds"                 | NodeDigitalObject   |  /com/lar/nodes/Digital1          0x18
//    "/etc/candaemon/eds/Digital.eds"                 | NodeD}igitalObject  |  /com/lar/nodes/Digital2          0x19
//    "/etc/candaemon/eds/lar_analognode_v15.eds"      | NodeAnalogObject    |  /com/lar/nodes/Analog1           0x2
//    "/etc/candaemon/eds/lar_doppelmotornode_v12.eds" | NodeMotor3Object    |  /com/lar/nodes/Doppelmotor1      0x12
//    "/etc/candaemon/eds/lar_doppelmotornode_v13.eds" | NodeMotor3Object    |  /com/lar/nodes/Doppelmotor2      0x14
//    "/etc/candaemon/eds/Scan.eds"                    | NodeObject          |  /com/lar/nodes/Scan      !Erzeugt in 'node_device_object_create_scan_node'
// <!-- GetIndexValue:
// @index:  Index .
//    @value:  Index value in string format.
//    @value:  Index value type in string format.
// @since: 1.10 Get intex value in string format-->
// <method name="GetIndexValue">
// <arg direction="in" type="s" name="index"/>
// <arg direction="out" type="s" name="value"/>
// <arg direction="out" type="s" name="value_type"/>
// </method>
// <!-- SetIndexValue:
// @index:    Index.
//    @value:    Intex value in string format.
//    @is_done:  Done if value changed.
// @since: 1.10 Set intex value-->
// <method name="SetIndexValue">
// <arg direction="in" type="s" name="index"/>
// <arg direction="in" type="s" name="value"/>
// <arg direction="out" type="b" name="is_done"/>
// </method>

pub struct Driver {
    pub route: Route,
}

impl Driver {
    pub fn connection(addr:&str) -> Result<Connection,CanError> {
        let conn = match addr {
            "system"  => Connection::new_system()?,
            "session" => Connection::new_session()?,
            _         => Connection::open_private(addr)?,
        };
        Ok(conn)
    }
    pub fn open(address: &str) -> Result<Driver,CanError> {
        let (send_can, recv_client ) = channel::bounded::<Message>(1);
        let (send_client, recv_can ) = channel::bounded::<Message>(1);
        let addr = address.to_owned();
        let _ = std::thread::spawn(move || {
            //Remote1 tcp:host=192.168.66.59,port=6666
            let c = Driver::connection(addr.as_str()).expect("Open dbus connection fail");
            let _ = c.register();
            while let Ok(mut msg) = recv_can.recv() {
                println!("MSG:{:?}",msg);
                let path  = dbus_path_from_node(msg.addr.node);
                let index = msg.addr.eds_index();
                if msg.is_write() {
                    let m = DBusMessage::new_method_call( "com.lar.service.can",path.as_str(), "com.lar.nodes.simple", "SetIndexValue").unwrap().append2(index,msg.data.to_string());
                    let r = c.send_with_reply_and_block(m, 2000).expect("send dbus message failed");

                }else {
                    let m = DBusMessage::new_method_call( "com.lar.service.can",path.as_str(), "com.lar.nodes.simple", "GetIndexValue").unwrap().append1(index);
                    let r = c.send_with_reply_and_block(m, 2000).expect("send dbus message failed");
                    msg.set_value(r.get1().unwrap_or(String::from("")));
                }
                send_can.send(msg).unwrap();
            }

        });
        // handle.join();
        let route = Route {
            sender: send_client,
            resiver: recv_client,
        };
        Ok(Driver{route })
    }
    pub fn transmit(&self,msg:Message) -> Result<Message,CanError> {
        self.route.sender.send(msg).unwrap();
        let msg = self.route.resiver.recv().unwrap();
        Ok(msg)
    }
}

impl crate::Datagramm for Driver {
    fn tramsmit(&self,msg:Message) -> Result<Message,CanError> {
        self.route.sender.send(msg).unwrap();
        let msg = self.route.resiver.recv().unwrap();
        Ok(msg)
    }
}

fn dbus_path_from_node(node:u32) -> String {
    match node {
        0x2  => String::from("/com/lar/nodes/Analog1"),
        0x12 => String::from("/com/lar/nodes/Doppelmotor1"),
        0x14 => String::from("/com/lar/nodes/Doppelmotor2"),
        0x18 => String::from("/com/lar/nodes/Digital1"),
        0x19 => String::from("/com/lar/nodes/Digital2"),
        0x1c => String::from("/com/lar/nodes/Analogext1"),
        _ => String::from("/com/lar/nodes/Scan")
    }
}
