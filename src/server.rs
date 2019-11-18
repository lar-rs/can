
use jsonrpc_core::{IoHandler, Result};
// use jsonrpc_derive::rpc;
use jsonrpc_tcp_server::ServerBuilder;
use super::can::{Can,Data,Addr};
use super::rpc::Msg;
// use async_std::prelude::*;
use async_std::task;
use async_std::io;

impl Msg for Can {
    fn read(&self,node : i32,index: i32,sub: u8) -> Result<Data> {
        Ok(Data{
            bytes: Vec::new(),
        })
    }
    fn write(&self,node:i32,index: i32,sub: u8,value: Vec<u8>) -> Result<()> {
		// self.write(addr: Addr, tx: Data)
		Ok(())
    }
}



pub async fn start(addr:&str) -> io::Result<()> {
	task::block_on(async {

		let mut io = IoHandler::default();
		let rpc = Can::open("vcan0").unwrap();

		io.extend_with(rpc.to_delegate());
		let server = ServerBuilder::new(io)
		.start(&addr.parse().unwrap())
		.expect("Server must start with no issues");
		server.wait()
	});
	Ok(())
}


