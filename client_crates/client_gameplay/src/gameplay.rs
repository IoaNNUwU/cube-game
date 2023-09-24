use bevy::prelude::*;

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(States)]
pub enum Render3DWorldState {
    #[default]
    DoNotRender,
    DoRender,
}

pub struct D3WorldPlugin;

impl Plugin for D3WorldPlugin {
    
    fn build(&self, app: &mut App) {
        
    }
    
}

fn spawn_3d_camera(
    
) {
    
}

fn spawn_block_on_message_from_server(
    
) {
    
}