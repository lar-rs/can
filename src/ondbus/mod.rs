pub mod dig;
pub mod uart;
pub mod adc;
pub mod stepper;
pub mod stirrer;
pub mod gp;
pub use self::dig::{DigOUT,DigIN};
pub use self::gp::GP;
pub use self::stepper::Stepper;
pub use self::stirrer::Stirrer;
pub use self::uart::Uart;
pub use self::adc::{ADC12,Temp};
use structopt::StructOpt;
use std::path::Path;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::time::Duration;
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
// use async_std::io;


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
// use std::str::FromStr;
// use super::message::Addr;



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
                    let _ = c.send_with_reply_and_block(m, 2000).expect("send dbus message failed");

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

pub struct AnalogNode<'a> {
    pub node: u32,
    pub ain: [ADC12<'a>;5],
    pub temp: [Temp<'a>;3],
    pub uart: [Uart<'a>;2],
}

impl <'a> AnalogNode<'a> {
    pub fn new(c:&Connection,node: u32) -> AnalogNode{

        let node_path = match node {
            0x2 => format!("/com/lar/nodes/Analog1"),
            _    => format!("/com/lar/nodes/Analog1"),
        };
        let ain = [
            ADC12::new(c,node_path.clone(),"GenIn1".to_owned()),
            ADC12::new(c,node_path.clone(),"GenIn2".to_owned()),
            ADC12::new(c,node_path.clone(),"GenIn3".to_owned()),
            ADC12::new(c,node_path.clone(),"GenIn4".to_owned()),
            ADC12::new(c,node_path.clone(),"GenIn5".to_owned()),
        ];
        let temp = [
            Temp::new(c,node_path.clone(),"Temperatur01".to_owned()),
            Temp::new(c,node_path.clone(),"Temperatur02".to_owned()),
            Temp::new(c,node_path.clone(),"Temperatur03".to_owned()),
        ];
        let uart = [
            Uart::new(&c,node_path.clone(),"com.lar.nodes.Analog1".to_owned(),1),
            Uart::new(&c,node_path.clone(),"com.lar.nodes.Analog1".to_owned(),2),
        ];
        AnalogNode { node, ain, temp, uart}
    }
}

// pub async fn unix_datagramm(driver:Driver,path:&Path) -> io::Result<()> {
    // Ok(())
// }
pub struct MotorNode<'a> {
    pub node: u32,
    pub gp: [GP<'a>;2],
    pub stepper: [Stepper<'a>;2],
    pub stirrer: [Stirrer<'a>;2],
    pub uart: [Uart<'a>;2]
}


impl<'a> MotorNode<'a> {
    pub fn new(c:&Connection,node: u32) -> MotorNode{
        let node_path = match node {
            0x18 => format!("/com/lar/nodes/Doppelmotor1"),
            0x19 => format!("/com/lar/nodes/Doppelmotor2"),
            _    => format!("/com/lar/nodes/Doppelmotor1"),
        };
        let gp  = [
            GP::new(&c,node_path.clone(),1),
            GP::new(&c,node_path.clone(),2)
        ];
        let stepper  = [
            Stepper::new(&c,node_path.clone(),1),
            Stepper::new(&c,node_path.clone(),2)
        ];
        let stirrer = [
            Stirrer::new(&c,node_path.clone(),1),
            Stirrer::new(&c,node_path.clone(),2),
        ];
        let uart = [
            Uart::new(&c,node_path.clone(),"com.lar.nodes.Doppelmotor3".to_owned(),1),
            Uart::new(&c,node_path.clone(),"com.lar.nodes.Doppelmotor3".to_owned(),2),
        ];
       MotorNode{ node, gp, stepper, stirrer, uart}

    }
}


pub struct Nodes<'a> {
    pub analog:  AnalogNode<'a>,
    pub motor:   [MotorNode<'a>;2],
    pub digital: [DigitalNode<'a>;2],
}


impl<'a> Nodes<'a> {
    pub fn new(c:&'a Connection)  -> Nodes<'a> {
        let analog = AnalogNode::new(c, 0x2);
        let motor = [
            MotorNode::new(c,0x12),
            MotorNode::new(c,0x14),
        ];
        let digital = [
            DigitalNode::new(c,0x18),
            DigitalNode::new(c,0x19),
        ];
        Nodes { analog, motor, digital }
    }
    // pub fn open<'a>(addr:&str) -> Nodes<'a> {
        // let c =
    // }
}

pub fn connection(address:&str) -> Result<Connection,CanError> {
    let conn = match address {
        "system"  => Connection::new_system().expect("connect to system bus failed"),
        "session" => Connection::new_session().expect("connect to session bus failed"),
        _         => Connection::open_private(address).expect("connect to private bus failed"),
    };
    conn.register()?;
    Ok(conn)
}


pub fn start(root:&Path,address:&str) -> Result<(),CanError> {
    let (tx, rx) = std::sync::mpsc::channel();
    // Automatically select the best implementation for your platform.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();
    // Automatically select the best implementation for your platform.
    watcher.watch(&root, RecursiveMode::Recursive).unwrap();
    let conn = connection(address)?;
    dig::setup_output(&conn,&root.join("digital1"),"/com/lar/nodes/Digital1")?;
    dig::setup_output(&conn,&root.join("digital2"),"/com/lar/nodes/Digital2")?;
    loop {
        match rx.recv() {
           Ok(event) => {
               println!("changed: {:?}", event);
               
           },
           Err(err) => println!("watch error: {:?}", err),
        };
    }
    
    Ok(())
}
