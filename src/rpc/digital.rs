use jsonrpc_core::{Result};
use jsonrpc_derive::rpc;


#[rpc]
pub trait Digital {
    #[rpc(name = "digital_info")]
    fn get_info(&self,node:i32) -> Result<String>;
    #[rpc(name = "get_inputs")]
    fn get_inputs(&self, node:i32) -> Result<u16>;
    #[rpc(name = "get_outputs")]
    fn get_outputs(&self, node:i32) ->Result<u16>;
    #[rpc(name = "set_outputs")]
    fn set_outputs(&self,node:i32,value:u16) -> Result<()>;
    #[rpc(name = "digital_get_in00")]
    fn get_input00(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in01")]
    fn get_input01(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in02")]
    fn get_input02(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in03")]
    fn get_input03(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in04")]
    fn get_input04(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in05")]
    fn get_input05(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in06")]
    fn get_input06(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in07")]
    fn get_input07(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in08")]
    fn get_input08(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in09")]
    fn get_input09(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in10")]
    fn get_input10(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in11")]
    fn get_input11(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in12")]
    fn get_input12(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in13")]
    fn get_input13(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in14")]
    fn get_input14(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_in15")]
    fn get_input15(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out00")]
    fn get_output00(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out01")]
    fn get_output01(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out02")]
    fn get_output02(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out03")]
    fn get_output03(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out04")]
    fn get_output04(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out05")]
    fn get_output05(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out06")]
    fn get_output06(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out07")]
    fn get_output07(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out08")]
    fn get_output08(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out09")]
    fn get_output09(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out10")]
    fn get_output10(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out11")]
    fn get_output11(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out12")]
    fn get_output12(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out13")]
    fn get_output13(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out14")]
    fn get_output14(&self, node:i32) ->Result<bool>;
    #[rpc(name = "digital_get_out15")]
    fn get_output15(&self, node:i32) ->Result<bool>;

    #[rpc(name = "digital_set_out00")]
    fn set_output00(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out01")]
    fn set_output01(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out02")]
    fn set_output02(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out03")]
    fn set_output03(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out04")]
    fn set_output04(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out05")]
    fn set_output05(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out06")]
    fn set_output06(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out07")]
    fn set_output07(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out08")]
    fn set_output08(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out09")]
    fn set_output09(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out10")]
    fn set_output10(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out11")]
    fn set_output11(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out12")]
    fn set_output12(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out13")]
    fn set_output13(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out14")]
    fn set_output14(&self, node:i32,value:bool) -> Result<()>;
    #[rpc(name = "digital_set_out15")]
    fn set_output15(&self, node:i32,value:bool) -> Result<()>;
}
