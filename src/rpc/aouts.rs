use jsonrpc_core::{Result};
use jsonrpc_derive::rpc;

#[rpc]
pub trait AOuts {
    #[rpc(name = "getCount")]
    fn outs_count(&self) -> Result<u8>;
    #[rpc(name = "getOut")]
    fn get_outs(&self, num: u8) -> Result<u16>;
    #[rpc(name = "setOut")]
    fn set_outs(&self, num: u8, val: u16) -> Result<()>;
}
