//! TCP echo server.
//!
//! To send messages, do:
//!
//! ```sh
//! $ nc localhost 8080
//! ```

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use async_std::print;
use pwa_can::can::*;
use async_std::net::UdpSocket;
use bincode::{deserialize, serialize};
// use serde_json::{from_bytes,to_vec};

// use async_std::sync::channel;



// async fn process(stream: TcpStream) -> io::Result<()> {
    // println!("Accepted from: {}", stream.peer_addr()?);

    // let (reader, writer) = &mut (&stream, &stream);
    // io::copy(reader, writer).await?;

    // Ok(())
// }


// fn main() -> io::Result<()> {
//     task::block_on(async {
//         let listener = TcpListener::bind("127.0.0.1:8080").await?;
//         println!("Listening on {}", listener.local_addr()?);
//         let mut incoming = listener.incoming();
//         let (st, r) = channel(1);
//         let (s, rf) = channel(1);
//         let handle = task::spawn(async move {
//             while let Some(msg) = r.recv().await {
//                 // print!("MSG").await;
//                 let ret = dev.message(msg).unwrap();
//                 s.send(ret);
//             }
//         });
//         while let Some(stream) = incoming.next().await {
//             task::spawn(async {
//                 println!("Accepted from: {}", stream.peer_addr().unwrap());
//                 let mut buf = vec![0u8; 1024];
//                 let (n, _) = stream.recv_from(&mut buf).await?;
//                 let buf stream.
//                 let (reader, writer) = &mut (&stream, &stream);

//             });
//         }
//         Ok(())
//     })
// }

fn main() -> io::Result<()> {
    task::block_on(async {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    println!("Listening on {}", socket.local_addr()?);
    let mut dev = Can::open("vcan0").unwrap();
    loop {
        let mut buf =Vec::new();
            let (n, peer) = socket.recv_from(&mut buf).await?;
            let msg: CanMsg = deserialize(&buf).unwrap();  
            let msg = dev.message(msg).unwrap();
            // let msg :CanMsg = serde_json::from_slice(uf.as_slice())?;
            // dev.
            let encoded: Vec<u8> = serialize(&msg).unwrap();
            let sent = socket.send_to(&encoded, &peer).await?;
            println!("Sent {} out of {} bytes to {}", sent, n, peer);
        }
    })
}
