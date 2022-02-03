use crate::game::{GameMode, GameState};
use bevy::prelude::*;

const READY: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVER: Color = Color::rgb(0.35, 0.75, 0.35);
const BACKGROUND: Color = Color::rgb(0.9, 0.9, 0.95);

pub struct MenuData {
    data: Entity,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameMode::Menu).with_system(setup))
            .add_system_set(SystemSet::on_update(GameMode::Menu).with_system(menu_system))
            .add_system_set(SystemSet::on_exit(GameMode::Menu).with_system(cleanup_menu));
    }
}

// The system to handle the menu button actions.
fn menu_system(
    mut state: ResMut<State<GameMode>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "GO!!".to_string();
                state.set(GameMode::Playing).unwrap();
            }
            Interaction::Hovered => {
                text.sections[0].value = "READY?".to_string();
                *color = HOVER.into();
            }
            Interaction::None => {
                text.sections[0].value = "START".to_string();
                *color = READY.into();
            }
        }
    }
}

// Initialize the menu.
// TODO: Fix flexbox alignment: move everything closer the the center vertically.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<GameState>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let data_id = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: BACKGROUND.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(25.0), Val::Percent(10.0)),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::FlexEnd,
                        margin: Rect::all(Val::Auto),
                        ..Default::default()
                    },
                    color: BACKGROUND.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: "high score: ".to_string(),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-bold.ttf"),
                                        font_size: 60.0,
                                        color: Color::ORANGE,
                                    },
                                },
                                TextSection {
                                    value: state.high_score.to_string(),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-bold.ttf"),
                                        font_size: 60.0,
                                        color: Color::GOLD,
                                    },
                                },
                                TextSection {
                                    value: "\nscore: ".to_string(),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-bold.ttf"),
                                        font_size: 60.0,
                                        color: Color::BLACK,
                                    },
                                },
                                TextSection {
                                    value: state.score.to_string(),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-bold.ttf"),
                                        font_size: 60.0,
                                        color: Color::GOLD,
                                    },
                                },
                            ],
                            alignment: TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                vertical: VerticalAlign::Center,
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::FlexEnd,
                        margin: Rect::all(Val::Auto),
                        ..Default::default()
                    },
                    color: BACKGROUND.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: READY.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Button",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 40.0,
                                        color: BACKGROUND,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                });
        })
        .id();

    commands.insert_resource(MenuData { data: data_id });
}

// Delete the menu data.
fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.data).despawn_recursive();
}
