use bevy::app::App;
use bevy::prelude::{Component, OnEnter, OnExit, Plugin};
use camera::CubeCameraPlugin;
use client_state::ClientState;

pub struct ClientGameplayPlugin;

impl Plugin for ClientGameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                CubeCameraPlugin,
            ))
            .add_systems(
                OnEnter(ClientState::InGame), (
                    spawn_client_gameplay,
                ))
            .add_systems(
                OnExit(ClientState::InGame), (
                    despawn_client_gameplay,
                ));
    }
}

#[derive(Component)]
pub struct ClientGameplayElementMarker;

fn spawn_client_gameplay() {

}

fn despawn_client_gameplay() {

}