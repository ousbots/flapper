use crate::player::Player;
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameMode {
    Menu,
    Playing,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Velocity {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GameState {
    pub score: i64,
    pub high_score: i64,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameMode::Menu)
            .insert_resource(GameState {
                score: 0,
                high_score: 0,
            })
            .add_system_set(SystemSet::on_enter(GameMode::Playing).with_system(setup))
            .add_system_set(SystemSet::on_exit(GameMode::Playing).with_system(game_end));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// Set the game scores at the end of the game.
fn game_end(
    _commands: Commands,
    mut mode: ResMut<State<GameMode>>,
    mut state: ResMut<GameState>,
    query: Query<&Player>,
) {
    for player in query.iter() {
        state.score = player.position.x;
        if state.score > state.high_score {
            state.high_score = state.score;
        }
    }

    mode.set(GameMode::Menu).unwrap();
}
