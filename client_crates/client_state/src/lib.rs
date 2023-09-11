use bevy::app::App;
use bevy::prelude::{Plugin, States};

#[derive(States)]
#[derive(Copy, Clone)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Hash)]
pub enum ClientState {
    #[default]
    Loading,
    Menu,
    InGame,
}

pub struct ClientStatePlugin;

impl Plugin for ClientStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ClientState>();
    }
}