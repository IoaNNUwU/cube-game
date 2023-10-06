use bevy::prelude::*;

#[derive(Default, Clone, Hash, Debug, Ord, PartialOrd, Eq, PartialEq, States)]
pub enum Render3DWorldState {
    #[default]
    DoNotRender,
    DoRender,
}

pub struct D3WorldPlugin;

impl Plugin for D3WorldPlugin {
    fn build(&self, _app: &mut App) {}
}

// fn spawn_3d_camera() {}

// fn spawn_block_on_message_from_server() {}
