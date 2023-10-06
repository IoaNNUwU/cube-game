use bevy::app::AppExit;
use bevy::prelude::*;

use client_state::ClientState;

mod main_menu;
mod settings_menu;

pub struct MenuUiElementsPlugin;

impl Plugin for MenuUiElementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_plugins((
                main_menu::MainMenuPlugin,
                settings_menu::SettingsMenuPlugin
            ))
            .add_systems(
                OnEnter(ClientState::Menu), (
                    initialize_menu_state,
                    spawn_2d_camera,
                ))
            .add_systems(
                Update, (
                    change_button_colors_based_on_interactions.run_if(in_state(ClientState::Menu)),
                    apply_button_action.run_if(in_state(ClientState::Menu)),
                ))
            .add_systems(
                OnExit(ClientState::Menu), (
                    deinitialize_menu_state,
                    despawn_2d_camera,
                ));
    }
}

pub const NORMAL_BUTTON: Color = Color::TEAL;
pub const HOVERED_BUTTON: Color = Color::SEA_GREEN;
pub const HOVERED_PRESSED_BUTTON: Color = Color::LIME_GREEN;
pub const PRESSED_BUTTON: Color = Color::GREEN;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);


#[derive(States)]
#[derive(Copy, Clone)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Hash)]
pub enum MenuState {
    #[default]
    None,

    Main,
    Settings,
}

#[derive(Component, Default)]
pub enum MenuButtonAction {
    #[default]
    None,

    MoveInWorld,
    MoveToSettings,
    MoveToMainMenu,
    Exit,
}

fn initialize_menu_state(
    mut menu_state: ResMut<NextState<MenuState>>
) {
    menu_state.set(MenuState::Main)
}

fn deinitialize_menu_state(
    mut menu_state: ResMut<NextState<MenuState>>
) {
    menu_state.set(MenuState::None)
}

#[derive(Component)]
struct SelectedButton;

fn change_button_colors_based_on_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedButton>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn apply_button_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut client_state: ResMut<NextState<ClientState>>,
) {
    for (interaction, button_action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match button_action {
                MenuButtonAction::None => {}
                MenuButtonAction::MoveInWorld => {
                    client_state.set(ClientState::InGame);
                    menu_state.set(MenuState::None);
                }
                MenuButtonAction::MoveToSettings => {
                    menu_state.set(MenuState::Settings);
                }
                MenuButtonAction::MoveToMainMenu => {
                    menu_state.set(MenuState::Main);
                }
                MenuButtonAction::Exit => {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}

#[derive(Component)]
struct MenuCameraMarker;

fn spawn_2d_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MenuCameraMarker));
}

fn despawn_2d_camera(
    mut commands: Commands,
    query: Query<Entity, With<MenuCameraMarker>>,
) {
    for camera in query.iter() {
        commands.entity(camera).despawn()
    }
}