use crate::game::{GameMode, Position, Velocity};
use bevy::prelude::*;

const FLAP_VELOCITY: i64 = 20;
const GRAVITY_FACTOR: i64 = 10;
const TRANSLATION_FACTOR: f32 = 1. / 15.;

#[derive(Component)]
pub struct Player {
    pub position: Position,
    pub velocity: Velocity,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameMode::Playing).with_system(setup))
            .add_system_set(SystemSet::on_update(GameMode::Playing).with_system(player_system))
            .add_system_set(SystemSet::on_update(GameMode::Playing).with_system(gravity_system));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(20.0, 20.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            position: Position { x: 0, y: 0 },
            velocity: Velocity { x: -20, y: 20 },
        });
}

fn player_system(keyboard: Res<Input<KeyCode>>, mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in query.iter_mut() {
        if keyboard.pressed(KeyCode::Up) {
            player.velocity.y += FLAP_VELOCITY;
        }

        player.position.x += player.velocity.x;
        player.position.y += player.velocity.y;

        let translation = &mut transform.translation;
        translation.y += player.velocity.y as f32 * TRANSLATION_FACTOR;
        translation.x += player.velocity.x as f32 * TRANSLATION_FACTOR;
    }
}

fn gravity_system(mut query: Query<&mut Player>) {
    for mut player in query.iter_mut() {
        player.velocity.y -= GRAVITY_FACTOR;
    }
}
