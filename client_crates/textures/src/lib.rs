use bevy::prelude::*;

mod associated_texture;

pub use associated_texture::*;

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_textures);
    }
}

fn setup_textures(
    asset_server: Res<AssetServer>
) {
    // asset_server.load_folder(BLOCKS_PATH).unwrap_or_else(|_| panic!("Unable to load {:?}", BLOCKS_PATH));
}