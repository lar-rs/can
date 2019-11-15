use jsonrpc_derive::rpc;
use jsonrpc_core::futures::future::{self, FutureResult};
use super::*;




#[rpc]
pub trait RpcMio {

    // fn (&self,) -> Result<String>;
    // #[rpc(name = "read_msg")]
    // fn get_analog_out(&self, num: u8) -> FutureResult<u16, Error>;
    #[rpc(name = "read_index")]
    fn read_index(&self,node:u32,index:u16, sub: u8) -> Result<String,MioError>;

    #[rpc(name = "write_index")]
    fn write_index(&self,node:u32,index:u16, sub: u8,data:Vec<u8>) -> Result<(),MioError>;

    #[rpc(name = "get_analog_in")]
    fn get_analog1_in(&self, num: u8) -> Result<u16, MioError>;

    #[rpc(name = "get_analog_out")]
    fn get_analog1_out(&self) -> Result<u16, MioError>;

    // #[rpc(name = "set_analog_out")]
    // fn set_analog_out(&self, num: u8, val: u16) -> FutureResult<(), MioError>;

    // #[rpc(name = "get_digital1_in")]
    // fn get_digita1_in(&self,digit:u8) ->Result<bool,MioError>;

    // #[rpc(name = "get_digital2_in")]
    // fn get_digita2_in(&self,digit:u8) ->Result<bool,MioError>;

    // #[rpc(name = "get_digital3_in")]
    // fn get_digita3_in(&self,digit:u8) ->Result<bool,MioError>;


    // #[rpc(name = "get_digital1_out")]
    // fn get_digita1_out(&self,digit:u8) ->Result<bool,MioError>;

    // #[rpc(name = "get_digital2_out")]
    // fn get_digita2_out(&self,digit:u8) ->Result<bool,MioError>;

    // #[rpc(name = "get_digital3_out")]
    // fn get_digita3_out(&self,digit:u8) ->Result<bool,MioError>;

    //  #[rpc(name = "set_digital1_out")]
    // fn set_digita1_out(&self,digit:u8,value:bool) ->Result<(),MioError>;

    // #[rpc(name = "set_digital2_out")]
    // fn set_digita2_out(&self,digit:u8,value:bool) ->Result<(),MioError>;

    // #[rpc(name = "set_digital3_out")]
    // fn set_digita3_out(&self,digit:u8,value:bool) ->Result<(),MioError>;





    // #[rpc(name = "airflow_in")]
    // fn airflow_in(&self) -> Result<Airflow,Error>;

    // #[rpc(name = "airflow_out")]
    // fn airflow_out(&self) -> Result<Airflow,Error>;

    // #[rpc(name = "humidity")]
    // fn humidity(&self) -> Result<Humidity,Error>;


    // #[rpc(name = "pressure")]
    // fn pressure(&self) -> Result<Pressure,Error>;

    // #[rpc(name = "indor_temperature")]
    // fn indoor_temperature(&self) -> Result<Temperature,Error>;

    // #[rpc(name = "outdoor_temperature")]
    // fn indoor_temperature(&self) -> Result<Temperature,Error>;

    // #[rpc(name = "sensor_signal")]
    // fn sensor_signal(&self,sensor:u64) -> Result<f64,Error>;
    //  #[rpc(name = "sensor_signal")]
    // fn sensor_setup(&self,sensor:u64) -> Result<(),Error>;

    // #[rpc(name = "ndir1_signal")]
    // fn ndir1_signal(&self) -> Result<f64,Error>;
    // #[rpc(name = "ndir2_signal")]
    // fn ndir2_signal(&self) -> Result<f64,Error>;
    // #[rpc(name = "zirox_signal")]
    // fn zirox_signal(&self) -> Result<f64,Error>;
    // #[rpc(name = "no_signal")]
    // fn no_signal(&self) -> Result<f64,Error>;


    // fn setup_uart01(&self,baut: u16)->Result<(),Error>;
    // fn setup_uart02(&self,baut: u16)->Result<(),Error>;
    // fn setup_uart03(&self,baut: u16)->Result<(),Error>;
    // fn setup_uart04(&self,baut: u16)->Result<(),Error>;
    // fn setup_uart05(&self,baut: u16)->Result<(),Error>;
    // fn setup_uart06(&self,baut: u16)->Result<(),Error>;

    // fn read_uart01(&self)->FutureResult<Vec<u8>,Error>;
    // fn read_uart02(&self)->FutureResult<Vec<u8>,Error>;
    // fn read_uart03(&self)->FutureResult<Vec<u8>,Error>;
    // fn read_uart04(&self)->FutureResult<Vec<u8>,Error>;
    // fn read_uart05(&self)->FutureResult<Vec<u8>,Error>;
    // fn read_uart06(&self)->FutureResult<Vec<u8>,Error>;

    // fn write_uart01(&self,data: Vec<u8>)->Result<(),Error>;
    // fn write_uart02(&self,data: Vec<u8>)->Result<(),Error>;
    // fn write_uart03(&self,data: Vec<u8>)->Result<(),Error>;
    // fn write_uart04(&self,data: Vec<u8>)->Result<(),Error>;
    // fn write_uart05(&self,data: Vec<u8>)->Result<(),Error>;
    // fn write_uart06(&self,data: Vec<u8>)->Result<(),Error>;

}




