use bevy::prelude::*;
use protocol::client2server::Client2ServerPacket;
use server_network::{Client2ServerEvent, Server2ClientEvent};

pub struct ServerGameplayPlugin;

impl Plugin for ServerGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            tick,
            print_if_client_gives_handshake,
        ));
    }
}

fn tick(

) {

}

fn print_if_client_gives_handshake(
    mut c2s_events: EventReader<Client2ServerEvent>,
) {
    for event in c2s_events.iter() {
        match &event.packet {
            Client2ServerPacket::HandShake(p) => {
                println!("Client `{}`#{} gives handshake", p.player_name, event.client_id)
            }
            _ => {}
        }
    }

}