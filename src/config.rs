use serde::{Deserialize, Serialize};
use async_std::io;
use std::path::Path;

/// Configuration
 #[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub gpio_path: String,
    /// tube diameter in milimeter
    pub tube_diameter: f64,
    /// pump speed factor [0-100]
    pub pump_speed: f64,
    /// pump_pin
    pub pump_pin: u64,
    // pub first_pin:u64,
    // pub second_pin:u64,
    // pub th_pin:u64,
}



impl Default for Config {
    fn  default() -> Config {
        Config {
            gpio_path:"/sys/class/gpio".to_owned(),
            tube_diameter: 5.0,
            pump_speed:100.0,
            pump_pin: 5,
        }
    }
}

/// read config
pub async fn read(_path: &Path) -> io::Result<Config> {
    // let config = fs::read_to_string(path).await?;
    Ok(Config::default())
}


pub async fn write(_config: Config) ->io::Result<()> {

    Ok(())
}
