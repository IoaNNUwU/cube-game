use basic::block::{BlockState, BlockType};
use basic::position::WorldBlockPosition;
use protocol::server2client::{FromServerHandShake, FromServerPacket, FromServerPlaceBlock};
use std::io::Write;
use std::time::Duration;

pub fn run_server() {
    let addr = "127.0.0.1:25533";

    let tcp_listener = std::net::TcpListener::bind(addr).expect("Unable to bind");

    loop {
        let (mut tcp_stream, _) = tcp_listener.accept().expect("Unable to accept");

        std::thread::spawn(move || {
            let mut i = 0;
            let current_thread_id = std::thread::current().id();

            let packet = FromServerPacket::HandShake(FromServerHandShake {
                server_name: "eblocraft".to_string(),
            });

            let bytes: Vec<u8> = serde_json::to_vec(&packet).expect(&format!(
                "Unable to write on thread {:?}",
                current_thread_id
            ));

            tcp_stream
                .write_all(bytes.as_slice())
                .expect("Unable to write bytes");
            println!(
                "WROTE {:?} on thread {:?}: {:?}",
                i,
                current_thread_id,
                String::from_utf8(bytes).unwrap()
            );

            let (mut x, mut y, mut z) = (1, 2, 3);

            loop {
                let packet = FromServerPacket::SetBlock(FromServerPlaceBlock {
                    block_state: BlockState {
                        block_type: BlockType::Dirt,
                        transformation: Default::default(),
                        data: Default::default(),
                    },
                    position: WorldBlockPosition::new(x, y, z),
                });

                let bytes: Vec<u8> = serde_json::to_vec(&packet).expect(&format!(
                    "Unable to write on thread {:?}",
                    current_thread_id
                ));

                tcp_stream
                    .write_all(bytes.as_slice())
                    .expect("Unable to write bytes");
                println!(
                    "WROTE {:?} on thread {:?}: {:?}",
                    i,
                    current_thread_id,
                    String::from_utf8(bytes).unwrap()
                );

                std::thread::sleep(Duration::from_secs(1));
                i += 1;

                x += 1;
                y += 1;
                z += 1;
            }
        });
    }
}
