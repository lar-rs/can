//! Serve json rpc TCP
//! 
//! 

// use jsonrpc_core::futures::future::{self, FutureResult};
// use jsonrpc_core::{IoHandler};
// use jsonrpc_derive::rpc;
// use jsonrpc_core::{IoHandler, Result};
// use jsonrpc_derive::rpc;
// use jsonrpc_tcp_server::ServerBuilder;
// use std::net::*;
// use async_std::task;

use log::info;
use async_std::prelude::*;
use async_std::io;
use async_std::eprintln;
use async_std::io::{BufReader,BufWriter};
// use crate::cli::Args;
// use duplexify::Duplex;
// use serde_json::to_string;


// use crate::rpc::*;
// use crate::rpc::server::*;
// use crate::can::*;
use crate::dbus::Driver;
use crate::message::{Message};
use structopt::StructOpt;
// use futures::prelude::*;

/// ✇ setup device
#[derive(Debug, StructOpt)]
pub struct Opt {
     /// Interface name
     #[structopt(name = "interface", default_value = "can0", long = "interface")]
     interface: String,
}

impl Opt {
    pub async fn run(&self) -> io::Result<()> {
        // This async scope times out after 5 seconds.
        // let iface =args.interface;


        info!("start pipe to:{}",self.interface);
        println!("{}",Message::help());
        let stdin = io::stdin();
        let stdout = io::stdout();
        let reader = BufReader::new(stdin);
        let mut writer = BufWriter::new(stdout);
        let mut lines = reader.lines();
        let device = Driver::open(self.interface.as_str())?;
        // let device = match self.interface.as_str() {
            // "can0" => Can::open(&self.interface)?,
            // _ => Can::open(&self.interface)?
        // };
        while let Some(line) =lines.next().await {
            let s = line?;
            match Message::from_str(&s) {
                Ok((_,msg)) => { 
                    println!("TX:{}",msg);
                    let msg = device.transmit(msg)?;
                    println!("RX:{}",msg);

                    writer.write(&msg.data.bytes).await?;
                },
                Err(e) => { 
                    // let msg = telegramm(Telegramm::Method(s.to_string())).await?;
                    // println!("METDOD:{:?}",msg);
                    eprintln!("Error: Could not compile to CanMsg:{:?}",e).await;
                    writer.write(b"e").await?;
                },
            }
            // if let = {
                // let data = can0.read(&addr)?;
            // }
        }
        Ok(())
    }
        // Read a line from the standard input and display it.

        // task::block_on(async {
            // let mut io = IoHandler::default();
            // let msg = CanNode;
            // let analog = AnalogNode;
            // let digital = DigitalNode;
            // let aouts   = AnalogOutputs;
            // let motor   = MotorNode;
            // io.extend_with(msg.to_delegate());
            // io.extend_with(analog.to_delegate());
            // io.extend_with(digital.to_delegate());
            // io.extend_with(aouts.to_delegate());
            // io.extend_with(motor.to_delegate());
            // let server = ServerBuilder::new(io)
            //     .start(&SocketAddr::new(self.address.parse().unwrap(),self.port))
            //     .expect("Server must start with no issues");
            // server.wait();
        // });
       // let dir = PathBuf::from(&self.path);
        // os::setup_config(&dir).await?;
        // os::enable_ssh(&dir).await?;
        // os::enable_wlan(&dir).await?;
        // Ok(())
    // }
}