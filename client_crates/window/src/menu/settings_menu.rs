use bevy::prelude::*;
use super::MenuState;

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(OnEnter(MenuState::Settings), spawn_settings_menu)
            .add_systems(OnExit(MenuState::Settings), despawn_settings_menu);
    }
}

fn spawn_settings_menu() {
    todo!()
}

fn despawn_settings_menu() {
    todo!()
}