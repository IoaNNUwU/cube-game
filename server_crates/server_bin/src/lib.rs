use bevy::app::App;
use bevy::MinimalPlugins;
use server_network::ServerNetworkPlugin;

pub fn run_server() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(ServerNetworkPlugin)
        .run();

}