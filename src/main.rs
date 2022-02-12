use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod game;
mod input;
mod menu;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(game::GamePlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

// Setup the cameras and the physics system.
fn setup(mut commands: Commands, mut physics: ResMut<RapierConfiguration>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    physics.scale = 50.;
}
