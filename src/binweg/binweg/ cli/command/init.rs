use structopt::StructOpt;
// use automata::error::*;
// use std::env::args;
use async_std::fs;
use async_std::io;
use async_std::prelude::*;
use async_std::task;

/// 📠  init root directory
#[derive(Debug,StructOpt)]
pub struct Opt {
    /// 🍱  root directory
    #[structopt(short = "p", long = "path",  default_value = ".")]
    path: String,
}

pub async fn run (opt: Opt)-> io::Result<()> {


    task::block_on(async {
        let mut dir = fs::read_dir(&opt.path).await?;

        while let Some(entry) = dir.next().await {
            println!("{}", entry?.file_name().to_string_lossy());
        }

        Ok(())
    })
}
