use bevy::prelude::*;
use protocol::client2server::Client2ServerPacket;
use protocol::server2client::{S2cLoadChunk, S2cPlayerMove, Server2ClientPacket};
use protocol::server2client::Server2ClientPacket::LoadChunk;
use server_network::{Client2ServerEvent, Server2ClientEvent};
use world::chunk::Chunk;

pub struct ServerGameplayPlugin;

impl Plugin for ServerGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            tick,
            print_if_client_gives_handshake,
        ));
    }
}

fn tick() {}

fn print_if_client_gives_handshake(
    mut c2s_events: EventReader<Client2ServerEvent>,
    mut s2c_events: EventWriter<Server2ClientEvent>,
) {
    for event in c2s_events.iter() {
        match &event.packet {
            Client2ServerPacket::HandShake(p) => {
                s2c_events.send(Server2ClientEvent {
                    client_id: event.client_id,
                    packet: Server2ClientPacket::ClientMoves(S2cPlayerMove {
                        id: event.client_id,
                        new_position: Default::default(),
                    }),
                });
                s2c_events.send(Server2ClientEvent {
                    client_id: event.client_id,
                    packet: LoadChunk(Default::default()),
                });
            }
            _ => {}
        }
    }
}