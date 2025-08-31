use crate::servers::area_server::VceAreaServer;
use crate::servers::auth_server::VceAuthServer;
use crate::servers::msg_server::VceMsgServer;

pub mod compression;
pub mod crypt;
pub mod net;
pub mod servers;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut auth = VceAuthServer::new("0.0.0.0", 50050);
    let mut msg = VceMsgServer::new("0.0.0.0", 50052);
    let mut area = VceAreaServer::new("0.0.0.0", 50054);
    loop {
        auth.tick();
        msg.tick();
        area.tick();
    }

    // let listener = TcpListener::bind("0.0.0.0:50050").expect("Failed to create listener");
    // println!("Listening on 0.0.0.0:50050");
    //
    // // Accept one connection (or use for loop to accept many)
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             println!(
    //                 "Client connected from {:?}",
    //                 stream.peer_addr().expect("No peer addres")
    //             );
    //             stream.set_nonblocking(true);
    //
    //             // Wrap TcpStream in VceStream
    //             let mut vce_peer = VcePeer::new(
    //                 NetworkStream::Tcp(stream),
    //                 CompressionType::None,
    //                 CryptType::None,
    //                 None
    //             );
    //
    //             // Handle connection in a new thread (optional)
    //             thread::spawn(move || {
    //                 if let Err(e) = handle_client(&mut vce_peer) {
    //                     eprintln!("Connection error: {:?}", e);
    //                 }
    //             });
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to accept connection: {:?}", e);
    //         }
    //     }
    // }
}

// fn handle_client(stream: &mut VcePeer) -> Result<(), NetError> {
//     loop {
//         match stream.tick() {
//             _ => (),
//             Err(e) => return Err(e),
//         }
//
//         // std::thread::sleep(std::time::Duration::from_millis(10));
//     }
//
//     Ok(())
// }
