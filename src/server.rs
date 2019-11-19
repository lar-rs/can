
use jsonrpc_core::{IoHandler, Result};
// use jsonrpc_derive::rpc;
use jsonrpc_tcp_server::ServerBuilder;
use super::can::{Can,Data,Addr};
use super::rpc::Msg;
// use async_std::prelude::*;
use async_std::task;
use async_std::io;




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


