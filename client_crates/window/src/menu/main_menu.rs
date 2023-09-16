use bevy::prelude::*;
use item::Item;
use item::solid_item::SolidBlockItem;
use textures::AssociatedTexture;

use super::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(MenuState::Main), (
                    spawn_main_menu,
                ))
            .add_systems(
                OnExit(MenuState::Main), (
                    despawn_main_menu,
                ));
    }
}

#[derive(Component)]
struct OnMainMenuMarker;

fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::BEIGE,
            custom_size: Some(Vec2::new(10000.0, 10000.0)),
            ..default()
        },
        ..default()
    }, OnMainMenuMarker));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Cube Game",
                            TextStyle {
                                font_size: 80.0,
                                color: Color::GRAY,
                                ..default()
                            },
                        )
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(50.0)),
                                ..default()
                            }),
                    );

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::MoveInWorld,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load(&*SolidBlockItem::PlainsLeaves.texture_path().unwrap());
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Play",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::MoveToSettings,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load(&*SolidBlockItem::Dirt.texture_path().unwrap());
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Exit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load(&*SolidBlockItem::Stone.texture_path().unwrap());
                            parent.spawn(ImageBundle {
                                style: button_icon_style,
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section("Exit", button_text_style));
                        });
                });
        });
}

fn despawn_main_menu(
    mut commands: Commands,
    loading_menu_ui_elements: Query<Entity, With<OnMainMenuMarker>>,
) {
    for entity in loading_menu_ui_elements.iter() {
        commands.entity(entity).despawn_recursive();
    }
}