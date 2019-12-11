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

// use std::prelude::*;
use std::io;
// use std::eprintln;
use std::io::{BufReader,BufWriter};
// use crate::cli::Args;
// use duplexify::Duplex;
// use serde_json::to_string;


// use crate::rpc::*;
// use crate::rpc::server::*;
// use crate::can::*;
use crate::io::Driver;
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
    pub fn run(&self) -> io::Result<()> {
        // This async scope times out after 5 seconds.
        // let iface =args.interface;

        log::info!("start pipe to:{}",self.interface);
        println!("{}",Message::help());
        let stdin = io::stdin();
        let stdout = io::stdout();
        let reader = BufReader::new(stdin);
        let mut writer = BufWriter::new(stdout);
        // let mut lines = reader.lines();
        let device = Driver::open(self.interface.as_str())?;
        // let device = match self.interface.as_str() {
            // "can0" => Can::open(&self.interface)?,
            // _ => Can::open(&self.interface)?
        // };
        // while let Some(line) =lines.next().await {
        //     let s = line?;
        //     match Message::from_str(&s) {
        //         Ok((_,msg)) => { 
        //             println!("TX:{}",msg);
        //             let msg = device.transmit(msg)?;
        //             println!("RX:{}",msg);

        //             writer.write(&msg.data.bytes).await?;
        //         },
        //         Err(e) => { 
        //             // let msg = telegramm(Telegramm::Method(s.to_string())).await?;
        //             // println!("METDOD:{:?}",msg);
        //             eprintln!("Error: Could not compile to CanMsg:{:?}",e).await;
        //             writer.write(b"e").await?;
        //         },
        //     }
            // if let = {
                // let data = can0.read(&addr)?;
            // }
        // }
        Ok(())
    }
}