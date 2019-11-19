use serde::{Deserialize, Serialize};
use async_std::io;
use async_std::fs::File;
use async_std::prelude::*;
use async_std::fs;
use async_std::path::Path;
use toml;
/// Configuration 
 #[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub iface: String,
}



impl Default for Config {
    fn  default() -> Config {
        Config {
            iface:"vcan".to_owned(),
        }
    }
}

/// read config
pub async fn read(path: &Path) -> io::Result<Config> {
    let toml_str = fs::read_to_string(path).await?;
    if let Ok(conf) = toml::from_str(&toml_str) {
        Ok(conf)
    }else {
        Ok(Config::default())
    }
}



pub async fn write(config: Config,path:&Path) ->io::Result<()> {
    let toml_str = toml::to_string(&config).unwrap();
    let mut file = File::create(path).await?;
    file.write_all(toml_str.as_bytes()).await?;
    file.sync_all().await?;
    Ok(())
}
