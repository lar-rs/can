#![allow(unused_variables)]

use failure::{Fail};
use std::io;
use std::io::{Error, ErrorKind};

// use std::string::FromUtf8Error;

use socketcan::{CANSocketOpenError,ConstructionError};
use dbus::Error as DBusError;
// use jsonrpc_core::Error as RpcError;
// use mut_guard::*;

#[derive(Fail, Debug)]
pub enum CanError {
    #[fail(display = "io error - {}",err)]
    IOError {err: io::Error },

    // #[fail(display = "rpc error - {}",err)]
    // RpcError {err: RpcError },

    #[fail(display = "dbus error - {}",err)]
    DBusError {err: DBusError },
     
    #[fail(display = "device error - {}",msg)]
    CanDev {msg: String},

    #[fail(display = "data error - {}",msg)]
    DataErr {msg: String},

   #[fail(display = "socketcan open error {}", err)]
    CanOpenError { err: CANSocketOpenError },

   #[fail(display = "socketcan frame construction error {}", err)]
    FrameConstructError{err:ConstructionError},

    #[fail(display = "socket can error - {}", msg)]
    Canbus { msg: String },

    #[fail(display = "socket can error - {}", msg)]
    Convert { msg: String },

}


pub fn wrong_data(msg:String) -> CanError {
    CanError::DataErr{ msg }
}

pub fn unknown_interface(msg:String) -> CanError {
    CanError::CanDev{ msg }
}

// impl From<RpcError> for CanError {
    // fn from(kind: RpcError) -> CanError {
        // CanError::RpcError{err:kind}
    // }
// }
impl From<CANSocketOpenError> for CanError {
    fn from(kind: CANSocketOpenError) -> CanError {
        CanError::CanOpenError{err:kind}
    }
}
impl From<ConstructionError> for CanError {
    fn from(kind: ConstructionError) -> CanError {
        CanError::FrameConstructError{err:kind}
    }
}
impl From<io::Error> for CanError {
    fn from(kind:io::Error) -> CanError {
        CanError::IOError{err: kind}
    }
}
impl From<CanError> for Error {
    fn from(canerr:CanError) -> Error {
        Error::new(ErrorKind::Other, format!("can error - {}",canerr))
    }
}

// impl From<CanError> for RpcError {
    // fn from(canerr:CanError) -> RpcError {
        // RpcError::internal_error()
    // }
// }
impl From<DBusError> for CanError {
    fn from(kind:DBusError) -> CanError {
        CanError::DBusError{err:kind}
    }
}
// 
// impl From<std::convert::From<std::string::String> for CanError {
    // fn from(kind:)
// }

//
