use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bevy::input::system::exit_on_esc_system)
            .add_system(keyboard_input);
    }
}

fn keyboard_input(input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Up) {}
}
