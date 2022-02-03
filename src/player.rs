use crate::game::{GameMode, Position, Velocity};
use bevy::prelude::*;

const FLAP_X_VELOCITY: i64 = 2;
const FLAP_Y_VELOCITY: i64 = 10;
const GRAVITY_FACTOR: i64 = 6;
const RESISTANCE_FACTOR: i64 = 1;
const TRANSLATION_FACTOR: f32 = 1. / 15.;

#[derive(PartialEq)]
enum State {
    Idle,
    Flapping,
    Gliding,
    Pecking,
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    pub position: Position,
    pub velocity: Velocity,
    state: State,
    direction: Direction,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameMode::Playing).with_system(setup))
            .add_system_set(SystemSet::on_update(GameMode::Playing).with_system(player_system))
            .add_system_set(SystemSet::on_update(GameMode::Playing).with_system(animate_system))
            .add_system_set(SystemSet::on_update(GameMode::Playing).with_system(physics_system));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/bird.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 9, 1);
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.)),
            ..Default::default()
        })
        .insert(Player {
            position: Position { x: 0, y: 0 },
            velocity: Velocity { x: 0, y: 20 },
            state: State::Gliding,
            direction: Direction::Right,
        })
        .insert(Timer::from_seconds(0.1, true));
}

// Manages the players velocity and position based on user input.
fn player_system(keyboard: Res<Input<KeyCode>>, mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in query.iter_mut() {
        if keyboard.pressed(KeyCode::Up) {
            player.velocity.y += FLAP_Y_VELOCITY;
            player.state = State::Flapping;
        }

        if keyboard.pressed(KeyCode::Left) {
            player.velocity.x -= FLAP_X_VELOCITY;
            player.state = State::Flapping;
            player.direction = Direction::Left;
        }

        if keyboard.pressed(KeyCode::Right) {
            player.velocity.x += FLAP_X_VELOCITY;
            player.state = State::Flapping;
            player.direction = Direction::Right;
        }

        if !keyboard.any_pressed(vec![KeyCode::Up, KeyCode::Left, KeyCode::Right])
            && player.state != State::Idle
            && player.state != State::Pecking
        {
            player.state = State::Gliding;
        }

        player.position.x += player.velocity.x;
        player.position.y += player.velocity.y;

        let translation = &mut transform.translation;
        translation.y += player.velocity.y as f32 * TRANSLATION_FACTOR;
        translation.x += player.velocity.x as f32 * TRANSLATION_FACTOR;
    }
}

// Animate the player sprite.
fn animate_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Player,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, player) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            match player.direction {
                Direction::Left => {
                    sprite.flip_x = false;
                }
                Direction::Right => {
                    sprite.flip_x = true;
                }
            }

            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            // sprite.index = (sprite.index + 1) % texture_atlas.textures.len();

            match player.state {
                State::Gliding => {
                    sprite.index = 3;
                }
                State::Flapping => {
                    sprite.index += 1;
                    if sprite.index >= texture_atlas.textures.len() {
                        sprite.index = 3;
                    }
                    if sprite.index < 3 {
                        sprite.index = 3;
                    }
                }
                State::Idle => {
                    sprite.index = 0;
                }
                State::Pecking => {
                    sprite.index = 0;
                }
            }
        }
    }
}

// Simulate gravity and air resistance on the players.
fn physics_system(mut query: Query<&mut Player>) {
    for mut player in query.iter_mut() {
        player.velocity.y -= GRAVITY_FACTOR;

        if player.velocity.x != 0 {
            if player.velocity.x > 0 {
                player.velocity.x -= RESISTANCE_FACTOR;
            } else {
                player.velocity.x += RESISTANCE_FACTOR;
            }
        }
    }
}
