use jsonrpc_ipc_server;
use jsonrpc_ipc_server::jsonrpc_core::{MetaIoHandler,Value};

// use can4rpc::*;
// use std::net::*;
// use std::io::prelude::*;

#[derive(structopt::StructOpt)]
struct Local {
    /// Address to listen on.
    #[structopt(short = "a", long = "address", default_value = "/tmp/can4linux.")]
    address: String,
}

fn main() {
	let mut io = MetaIoHandler::<()>::default();
	io.add_method("say_hello", |_params| Ok(Value::String("hello".to_string())));
	let _server = jsonrpc_ipc_server::ServerBuilder::new(io)
		.start("/tmp/wqm-mio.ipc")
		.expect("Server should start ok");
}

