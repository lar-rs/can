use serde::{Deserialize, Serialize};
use std::io;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
// use std::prelude::*;
use std::path::Path;
use toml;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Index {
    pub addr: u32,
}

/// Configuration 
 #[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub iface: String,
    pub bautrate: u32,

}



impl Default for Config {
    fn  default() -> Config {
        Config {
            iface:"vcan".to_owned(),
            bautrate: 125000,
        }
    }
}

/// read config
pub fn read(path: &Path) -> io::Result<Config> {
    let toml_str = fs::read_to_string(path)?;
    if let Ok(conf) = toml::from_str(&toml_str) {
        Ok(conf)
    }else {
        Ok(Config::default())
    }
}

pub fn write(config: Config,path:&Path) ->io::Result<()> {
    let toml_str = toml::to_string(&config).unwrap();
    let mut file = File::create(path)?;
    file.write_all(toml_str.as_bytes())?;
    file.sync_all()?;
    Ok(())
}
