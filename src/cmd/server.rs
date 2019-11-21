//! Serve json rpc TCP
//! 
//! 
use jsonrpc_core;

// use jsonrpc_core::futures::future::{self, FutureResult};
use jsonrpc_core::{IoHandler};
// use jsonrpc_derive::rpc;
// use jsonrpc_core::{IoHandler, Result};
// use jsonrpc_derive::rpc;
use jsonrpc_tcp_server::ServerBuilder;
use std::net::*;
use log::info;
// use std::io::prelude::*;
// use async_std::prelude::*;
// use async_std::task;
use async_std::io;
use crate::rpc::*;
use crate::rpc::server::*;


use structopt::StructOpt;
// use futures::prelude::*;

///  ✇ serve tcp/udp controller on net
#[derive(Debug, StructOpt)]
pub struct Opt {
     /// ip addres
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    iface: String,

    /// port number 
    #[structopt(short = "p", long = "port", env = "PORT", default_value = "6677")]
    port: u16,
     /// ip addres
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    address: String,
}

impl Opt {
    pub async fn run(&self, iface:&str) -> io::Result<()> {
        info!("Start rpc server {}:{}",self.port,self.address);
        // task::block_on(async {
            let mut io = IoHandler::default();
            let msg = CanNode;
            let analog = AnalogNode;
            let digital = DigitalNode;
            let aouts   = AnalogOutputs;
            let motor   = MotorNode;
            io.extend_with(msg.to_delegate());
            io.extend_with(analog.to_delegate());
            io.extend_with(digital.to_delegate());
            io.extend_with(aouts.to_delegate());
            io.extend_with(motor.to_delegate());
            let server = ServerBuilder::new(io)
                .start(&SocketAddr::new(self.address.parse().unwrap(),self.port))
                .expect("Server must start with no issues");
            server.wait();
        // });
       // let dir = PathBuf::from(&self.path);
        // os::setup_config(&dir).await?;
        // os::enable_ssh(&dir).await?;
        // os::enable_wlan(&dir).await?;
        Ok(())
    }
}