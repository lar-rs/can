// #![feature(async_await)]
use jsonrpc_tcp_server::jsonrpc_core::IoHandler;
use jsonrpc_tcp_server::ServerBuilder;
// use jsonrpc_tcp_server::jsonrpc_core::{IoHandler};
// use jsonrpc_tcp_server::{AccessControlAllowOrigin, DomainsValidation, RestApi, ServerBuilder};
// use jsonrpc_pubsub::{PubSubHandler, Session, Subscriber, SubscriptionId};

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

#[derive(structopt::StructOpt)]
struct Address {
    /// Port to listen on.
    #[structopt(short = "p", long = "port", env = "PORT", default_value = "3030")]
    port: u16,
    /// Address to listen on.
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    address: String,
}



//run comand
#[derive(structopt::StructOpt)]
enum Server {
    #[structopt(name = "http", about = "Run http server")]
    Http(Address),
    #[structopt(name = "ws", about = "Run over websocket")]
    WS(Address),
     #[structopt(name = "ipc", about = "Run over ipc")]
    TCP(Address),
    // #[structopt(name = "run", about = "Start running the bot")]
    // Run(run::Opt),
    // #[structopt(name = "setup", about = "")]
    // Setup(setup::Opt),
    // #[structopt(name = "twitter-list-sync", about = "")]
    // TwitterListSync(twitter_list_sync::Opt),
    // #[structopt(name = "twitter-login", about = "")]
    // TwitterLogin(twitter_login::Opt),
}


#[paw::main]
fn main(args:Args) -> Result<(), std::io::Error> {
    let mut io = IoHandler::new();
	io.extend_with(RpcServe.to_delegate());
    io.extend_with(DigitalIO.to_delegate());
	let server = ServerBuilder::new(io)
		.start(&SocketAddr::new(args.address.parse().unwrap(),args.port))
		.expect("Unable to start RPC server");

	server.wait();
    Ok(())
}




//
