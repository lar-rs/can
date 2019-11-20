use serde::{Serialize,Deserialize}





pub struct Settitng {
    pub bautrate: u32,
}



/// Setup pcan driver.
pub async setup_pcan() -> io::Result {

    // PCan usb device
    // ```sh
    // modprobe peak_usb 
    // sudo ip link set can0 up type can bitrate 500000
    // cargo test
    // ```
}

pub async get_bautrate() -> u32 {

}



