use bevy::prelude::*;

use bevy::DefaultPlugins;
use client_network::ClientNetworkPlugin;
use client_state::ClientStatePlugin;
use gameplay::GameplayPlugin;
use textures::TexturesPlugin;
use window::CubeWindowPlugin;

pub fn run_client() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest())
        )
        .add_plugins((
            ClientStatePlugin,
            ClientNetworkPlugin,
            TexturesPlugin,
            CubeWindowPlugin,
            GameplayPlugin,
        ))
        .run();
}

fn setup() {

}

fn update() {

}






