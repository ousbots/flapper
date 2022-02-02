use bevy::prelude::*;

mod button;
mod input;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameMode {
    Menu,
    Playing,
    End,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(button::ButtonPlugin)
        .add_plugin(input::InputPlugin)
        .add_state(GameMode::Menu)
        .add_startup_system(setup)
        .run();
}

// Setup the cameras.
fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
