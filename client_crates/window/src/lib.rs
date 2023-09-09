mod loading_screen;
mod menu;

use bevy::prelude::*;

pub struct CubeWindowPlugin;

impl Plugin for CubeWindowPlugin {
    fn build(&self, app: &mut App) {
        // Insert as resource the initial value for the settings resources
        app.insert_resource(Volume(7))
            .insert_resource(DisplayQuality::Medium)
            .add_state::<ClientState>()
            .add_plugins((
                loading_screen::LoadingScreenUIElementsPlugin,
                menu::MenuUiElementsPlugin,
            ));
    }
}

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

#[derive(States)]
#[derive(Copy, Clone)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Hash)]
pub enum LoadingState {
    #[default]
    None,

    LoadingMainMenu,
    LoadingWorld,
}

#[derive(States)]
#[derive(Copy, Clone)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Hash)]
pub enum InGameState {
    #[default]
    None,

    CameraControl,
    InsideInventory,
}

#[derive(States)]
#[derive(Copy, Clone)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Hash)]
pub enum InventoryState {
    #[default]
    None,

    Player,
    Workbench,
    Furnace,
    Chest,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);