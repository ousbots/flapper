use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameMode {
    Menu,
    Playing,
    Finish,
}

pub struct GameState {
    pub score: i64,
    pub high_score: i64,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameMode::Menu).insert_resource(GameState {
            score: 0,
            high_score: 0,
        });
    }
}
