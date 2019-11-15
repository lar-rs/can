pub mod check;
pub mod init;


use structopt::StructOpt;

/// 📣 The various kinds of commands that `cantorpc` can execute.
#[derive(Debug, StructOpt)]
pub enum Comd {
    /// 🔧 init automata directory
    #[structopt(name = "init", about = "📠  checkt CAN and nodes")]
    Init(init::Opt),
    /// 📠  checkt CAN and nodes.
    #[structopt(name = "check", about = "📠  checkt CAN and nodes")]
    Check(check::Opt),
   //  ⥄‍♀ run ipc communication mod
    // #[structopt(name = "ipc", about = "⥄‍♀ run ipc named socket server")]
    // Ipc(Opt),
   //  ⥄‍♀ run pipe mod.
    // #[structopt(name = "pipe", about = " ⥄‍♀ run in out pipe")]
    // Pipe,
   //  🔂‍♀ run tcp server.
    // #[structopt(name = "tcp", about = " 🔂‍♀ run tcp socket server")]
    // TCP(Opt),
   //  🔂‍♀ run udp server.
    // #[structopt(name = "udp", about = "🔂‍♀ run udp socket server")]
    // UDP(Opt),
}

/// 📣 The various kinds of commands that `automata` can execute.
#[derive(Debug, StructOpt)]
pub enum Command {

   ///  ⛽  run waterpipe .
    #[structopt(name = "start", about = "⛽ start waterpipe system")]
    Start{
        /// ⥄‍  direction [straight,garochi,tomato,basili]
        #[structopt(name = "direction", long = "direction", default_value = "straight")]
        direction:String,
        /// ⏱  interval in seconds
        #[structopt(name = "duration", long = "duration", default_value = "0")]
        duration: u64,

    },
    ///  ✇ start controller stdin stdout
    Pipe,
    ///  ✇ serve controller on net
    #[structopt(name = "serve", about = " ✇ start net controller")]
    Serve{
        /// 📪  linux socket file or ip addresse
        #[structopt(name = "address", long = "address")]
        address: String,
          /// 📪  linux socket file or ip addresse
        #[structopt(name = "port", long = "port")]
        port: u64,
    },


}

/// 📣 run automata kinds of commands 
pub async fn run (command: Command ) -> io::Result<()> {

    Ok(())
}
