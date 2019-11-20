//! Echoes lines read on stdin to stdout.

// use log::info;
// use async_std::io;
// use async_std::prelude::*;
// use async_std::task;
// use async_log::{instrument, span};

// use lscan::can::*;
// use lscan::cli::Args;
// fn main() -> io::Result<()> {

//     task::block_on(async {
//         let stdin = io::stdin();
//         let mut stdout = io::stdout();
//         let mut line = String::new();
//         let mut dev = Can::open("vcan0").unwrap();
//         loop {
//             // Read a line from stdin.
//             let n = stdin.read_line(&mut line).await?;
            
//             // If this is the end of stdin, return.
//             if n == 0 {
//                 return Ok(());
//             }

//             // Write the line to stdout.
//             stdout.write_all(line.as_bytes()).await?;
//             stdout.flush().await?;
//             line.clear();
//         }
//     })
// }
use lscan::cli::Args;
// use lscan::banner;

use async_log::span;
use log::info;
use async_std::io;
use async_std::task;

fn setup_logger() {
    let logger = femme::pretty::Logger::new();
    async_log::Logger::wrap(logger, || 12)
        .start(log::LevelFilter::Trace)
        .unwrap();
}

#[paw::main]
fn main(args: Args) -> io::Result<()> {
	lscan::banner::hello();
    setup_logger();
    // span!("new level, depth={}", 1, {
    //     let x = "beep";
    //     info!("look at this value, x={}", x);

    //     span!("new level, depth={}", 2, {
    //         let y = "boop";
    //         info!("another nice value, y={}", y);
    //     })
    // });
    task::block_on(args.command())
}

// fn setup_logger() {
    // let logger = femme::pretty::Logger::new();
    // async_log::Logger::wrap(logger, || /* get the task id here */ 0)
        // .start(log::LevelFilter::Trace)
//         // .unwrap();
// }

// #[paw::main]
// fn main(args: Args) -> Result<(), std::io::Error> {
    // setup_logger();
    
    // Ok(())
// }

// use jsonrpc_core;

// use jsonrpc_core::futures::future::{self, FutureResult};
// use jsonrpc_core::{Error, IoHandler, Result};
// use jsonrpc_derive::rpc;

// #[rpc]
// pub trait Rpc<One, Two> {
// 	/// Get One type.
// 	#[rpc(name = "getOne")]
// 	fn one(&self) -> Result<One>;

// 	/// Adds two numbers and returns a result
// 	#[rpc(name = "setTwo")]
// 	fn set_two(&self, a: Two) -> Result<()>;

// 	/// Performs asynchronous operation
// 	#[rpc(name = "beFancy")]
// 	fn call(&self, a: One) -> FutureResult<(One, Two), Error>;
// }

// struct RpcImpl;

// impl Rpc<u64, String> for RpcImpl {
// 	fn one(&self) -> Result<u64> {
// 		Ok(100)
// 	}

// 	fn set_two(&self, x: String) -> Result<()> {
// 		println!("{}", x);
// 		Ok(())
// 	}

// 	fn call(&self, num: u64) -> FutureResult<(u64, String), Error> {
// 		crate::future::finished((num + 999, "hello".into()))
// 	}
// }

// fn main() {
// 	let mut io = IoHandler::new();
// 	let rpc = RpcImpl;

// 	io.extend_with(rpc.to_delegate())
// }
