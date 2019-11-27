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
use can::cli::Args;
// use async_log::{instrument, span};
// use log::info;
// use lscan::banner;

// use async_log::span;
// use log::info;
use async_std::io;
use async_std::task;
use yansi::Paint;

/// setup logger use femme;
fn setup_logger() {
    let logger = femme::pretty::Logger::new();
    async_log::Logger::wrap(logger, || /* get the task id here */ 0)
        .start(log::LevelFilter::Info)
        .unwrap();
    // let logger = femme::pretty::Logger::new();
    // femme::start(log::LevelFilter::Trace).unwrap();
    // async_log::Logger::wrap(logger, || 12)
        // .start(log::LevelFilter::Trace)
        // .unwrap();
}

#[paw::main]
fn main(args: Args) -> io::Result<()> {
    println!("{}",Paint::blue(can::banner::NAME));
    setup_logger();
    task::block_on(args.command())
}