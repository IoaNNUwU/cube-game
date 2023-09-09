use bevy::prelude::*;
use bevy::sprite::{Anchor, SpriteBatch};
use basic::block::BlockType::*;
use textures::AssociatedTexture;

use crate::ClientState;

pub struct LoadingScreenUIElementsPlugin;

impl Plugin for LoadingScreenUIElementsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(ClientState::Loading), (
                    spawn_2d_camera,
                    spawn_main_menu_loading_screen,
                ))
            .add_systems(
                Update, (
                    tick_timer_and_change_game_state_to_menu_when_finished
                        .run_if(in_state(ClientState::Loading)),
                    rotate_image
                        .run_if(in_state(ClientState::Loading)),
                ))
            .add_systems(
                OnExit(ClientState::Loading), (
                    despawn_2d_camera,
                    despawn_main_menu_loading_screen,
                ));
    }
}

#[derive(Component)]
struct LoadingScreenCameraMarker;

fn spawn_2d_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), LoadingScreenCameraMarker));
}

fn despawn_2d_camera(
    mut commands: Commands,
    query: Query<Entity, With<LoadingScreenCameraMarker>>,
) {
    for camera in query.iter() {
        commands.entity(camera).despawn()
    }
}

#[derive(Component)]
struct OnLoadingScreenMarker;

#[derive(Component)]
struct RotatingImageMarker;

#[derive(Resource, Deref, DerefMut)]
struct LoadingTimer(Timer);

fn spawn_main_menu_loading_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let image_handle: Handle<Image> = asset_server.load(StoneBricks.texture_path());

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::BEIGE,
            custom_size: Some(Vec2::new(10000.0, 10000.0)),
            ..default()
        },
        ..default()
    }, OnLoadingScreenMarker));

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        OnLoadingScreenMarker,
    ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn((
                    ImageBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        image: UiImage::new(image_handle),
                        ..default()
                    },
                    RotatingImageMarker
                ));
            });
        });

    commands.insert_resource(LoadingTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn rotate_image(
    mut images: Query<&mut Transform, With<RotatingImageMarker>>,
    time: Res<Time>,
) {
    let rotation_speed = match (time.elapsed().as_millis() / 150) % 10 {
        0 => 0.05,
        1 | 9 => 0.06,
        2 | 8 => 0.075,
        3 | 7 => 0.090,
        4 | 6 => 0.110,
        5 => 0.120,
        _ => unreachable!()
    };


    let mut loading_menu_icon = images.single_mut();

    loading_menu_icon.rotate_around(Vec3::ZERO, Quat::from_rotation_z(rotation_speed));
}

fn tick_timer_and_change_game_state_to_menu_when_finished(
    mut game_state: ResMut<NextState<ClientState>>,
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(ClientState::Menu);
    }
}

fn despawn_main_menu_loading_screen(
    mut commands: Commands,
    loading_menu_ui_elements: Query<Entity, With<OnLoadingScreenMarker>>,
) {
    for entity in loading_menu_ui_elements.iter() {
        commands.entity(entity).despawn_recursive();
    }
}