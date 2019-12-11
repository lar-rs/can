
use async_std::io;
use async_std::fs;
use async_std::path::PathBuf;


/// PCan usb device `Peak`
/// * Linux modul activate:
/// modprobe peak_usb 
/// * Link vcan device: 
/// sudo ip link set can0 up type can bitrate 500000
use std::time::Duration;


pub async fn setup_index(&self, path:&Path) io::Result<()>{
    let mut file = File::create("node").await?;
    file.write_all(self.node.as_str()).await?;
    let mut file = File::create("index").await?;
    file.write_all(self.index.as_str()).await?;
}

pub async fn digital_output(path:&Path,node:String) {
    let index = Index::new(node,"6100sub01".to_owned())
    for n in  1..16 {
        let mut file = File::create(format!().await?;
        file.write_all(self.node.as_str()).await?;
        let path = path.join(format!("dout{}",num);
        if !path.is_dir().async?{
            fs.create_all().async?
            
        }
        let path = path.join(format!("din{}",num);
        if !path.is_dir().async?{
            fs.create_all().async?
        }

    }
}

pub async fn setup(path:&Path) -> io::Result<()> {
    let path = path.join("dbus");
    if !path.is_dir().async?{
        fs.create_all().async?
        dout().await
    }
}

pub async fn start(path:&Path) {
    setup(path).await?
}

