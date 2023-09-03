use protocol::server2client::FromServerPacket;
use std::io::Read;
use std::net::TcpStream;

pub fn run_client() {
    let addr = "127.0.0.1:25533";

    let mut tcp_stream = TcpStream::connect(addr).expect("Unable to connect");

    let mut buf: [u8; 8162] = std::array::from_fn(|_| 0);

    loop {
        let read_size = tcp_stream.read(&mut buf).expect("Unable to read");

        let buf_cut = buf.iter().take(read_size).copied().collect::<Vec<_>>();

        let Ok(packet) = serde_json::from_slice::<FromServerPacket>(buf_cut.as_slice()) else {
            continue;
        };

        match packet {
            FromServerPacket::HandShake(packet) => {
                println!("Handshake {}", packet.server_name);
            }
            FromServerPacket::SetBlock(packet) => {
                println!("SetBlock {:?} on {:?}", packet.block_state, packet.position);
            }
        };
    }
}
