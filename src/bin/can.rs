//! Can cli
use can::cli::Args;
// use async_log::{instrument, span};
// use log::info;
// use lscan::banner;

// use async_log::span;
// use log::info;
use std::io;
use yansi::Paint;

#[paw::main]
fn main(args: Args) -> io::Result<()> {
    println!("{}",Paint::blue(can::banner::NAME));
    args.command()
}


