use jsonrpc_ws_server::jsonrpc_core::{IoHandler};
use jsonrpc_ws_server::ServerBuilder;


use can4rpc::*;
use std::net::*;
use std::io::prelude::*;
// use std::io::prelude::*;
// use structopt;
// With the "paw" feature enabled in structopt

// #[derive(Debug, StructOpt)]
// #[structopt(name = "example", about = "An example of StructOpt usage.")]
#[derive(structopt::StructOpt)]
struct Args {
    /// Port to listen on.
    #[structopt(short = "p", long = "port", env = "PORT", default_value = "3030")]
    port: u16,
    /// Address to listen on.
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    address: String,
}

#[paw::main]
fn main(args:Args) -> Result<(), std::io::Error> {
    let mut io = IoHandler::default();
	io.extend_with(RpcServe.to_delegate());
	let server = ServerBuilder::new(io)
        .start(&SocketAddr::new(args.address.parse().unwrap(),args.port))
		.expect("Unable to start Can4RPC server");

	server.wait().unwrap();
    Ok(())
}
