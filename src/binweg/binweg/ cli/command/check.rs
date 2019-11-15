use prettytable::{
        Table,
        Row,Cell,
        table,
        row,
        cell,
 };

// use console::{style, Term};
use failure::{Fallible};
use structopt::StructOpt;
// use crate::can;

/// 📠  Check IO module , CAN and nodes.
#[derive(Debug,StructOpt)]
pub struct Opt {
    /// 🍱  node id to check or 0 to check all.
    #[structopt(short = "n", long = "node",  default_value = "0")]
    node: i32,

    #[structopt(long = "uv")]
    uv: bool,

    #[structopt(long = "ndirs")]
    ndir: bool,

    #[structopt(long = "airflow")]
    airflow: bool,



}

// pub fn analog_table(node: i32) -> Table {
//     let mut table = Table::new();
//     table.add_row(row![format!("Analog ID:{}",node)]);
//     table.add_row(row!["IN01",format!("{}",analog.get_input01())]);
//     table.add_row(row!["IN02",format!("{}",analog.get_input02())]);
//     table.add_row(row!["IN03",format!("{}",analog.get_input03())]);
//     table.add_row(row!["IN04",format!("{}",analog.get_input04())]);
//     table.add_row(row!["IN05",format!("{}",analog.get_input05())]);
//     table.add_row(row!["OUT",format!("{}",analog.get_out())]);
//     table.add_row(row!["TEMP01",format!("{}",analog.get_temp01())]);
//     table.add_row(row!["TEMP02",format!("{}",analog.get_temp02())]);
//     table.add_row(row!["TEMP03",format!("{}",analog.get_temp03())]);
//     table.add_row(row!["UART01",format!("{}",analog.get_uart01())]);
//     table.add_row(row!["UART02",format!("{}",analog.get_temp02())]);

//     table
// }
// pub fn nodes_table() -> Table {
//     let table = table!([analog_table(0x2)]);
//     table
// }
// pub fn uv_table() -> Table {
//     let mut table = Table::new();
//     table.add_row(row!["Pump",format!("{}",false)]);
//     table
// }

// pub fn ndir_table() -> Table {
//     let mut table = Table::new();
//     table.add_row(row!["Model","FSR","Uart"]);
//     table.add_row(row!["Unknown",format!("{}",0.0),format!("{}","eaafd".to_owned())]);
//     table

// }

// pub fn airflow_table() -> Table {
//     let mut table = Table::new();
//     table.add_row(row!["Air IN",format!("{}",0.0)]);
//     table.add_row(row!["Air OUT",format!("{}",0.0)]);
//     table.add_row(row!["Humidity",format!("{}",0.0)]);
//     table.add_row(row!["Pressure",format!("{}",0.0)]);
//     table

// }



// use dbus::blocking::{Connection,Proxy};
use std::time::Duration;

pub fn run (opt: Opt) ->Fallible<()>{
    // let conn = Connection::new_system()?;
    // let obj = conn.with_path("org.freedesktop.DBus", "/", 5000);
    // let (names,): (Vec<String>,) = obj.method_call("org.freedesktop.DBus", "ListNames", ())?;
    // for name in names { println!("{}", name); }

    // let mut table =  Table::new();
    // let nodes = match opt.node{
        // _ => nodes_table(),
    // };
    // table.add_row(row![nodes]);
    // if opt.uv {
    //     table.add_row(row![uv_table()]);
    // }
    // if opt.ndir {
    //     table.add_row(row![ndir_table()]);
    // }
    // if opt.airflow {
    //     table.add_row(row![airflow_table()]);
    // }
    // // Print the table to stdout
    // table.printstd();

    Ok(())
}
