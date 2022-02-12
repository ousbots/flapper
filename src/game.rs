use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameMode {
    Menu,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameMode::Menu)
            .add_system_set(SystemSet::on_enter(GameMode::Playing).with_system(setup))
            .add_system_set(SystemSet::on_exit(GameMode::Playing).with_system(game_end));
    }
}

// Setup game camera.
fn setup(mut commands: Commands) {
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // TODO: move platform setup to another module.
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(150., 20., 1.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.5, 0.2).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete);
}

// Change the game mode to menu when the game ends.
fn game_end(_commands: Commands, mut mode: ResMut<State<GameMode>>) {
    mode.set(GameMode::Menu).unwrap();
}
