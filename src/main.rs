use bevy::prelude::*;

mod game;
mod input;
mod menu;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(game::GamePlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

// Setup the cameras.
fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
