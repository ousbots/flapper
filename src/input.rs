use bevy::app::AppExit;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_input.label("input"));
    }
}

fn keyboard_input(input: Res<Input<KeyCode>>, mut endgame_writer: EventWriter<AppExit>) {
    if input.pressed(KeyCode::Escape) {
        endgame_writer.send(AppExit);
    }
}
