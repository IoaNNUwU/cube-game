use bevy::app::App;
use bevy::prelude::{Component, OnEnter, OnExit, Plugin};
use camera::CubeCameraPlugin;
use client_state::ClientState;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                CubeCameraPlugin,
            ))
            .add_systems(
                OnEnter(ClientState::InGame), (
                    spawn_gameplay,
                ))
            .add_systems(
                OnExit(ClientState::InGame), (
                    despawn_gameplay,
                ));
    }
}

#[derive(Component)]
pub struct GamePlayElementMarker;

fn spawn_gameplay() {}

fn despawn_gameplay() {}