use crate::game::GameMode;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const FLAP_X_FORCE: f32 = 1.5;
const FLAP_Y_FORCE: f32 = 3.5;

#[derive(PartialEq)]
enum State {
    Idle,
    Flapping,
    Gliding,
    Walking,
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    state: State,
    direction: Direction,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameMode::Playing).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameMode::Playing)
                    .with_system(player_system)
                    .label("player"),
            )
            .add_system_set(
                SystemSet::on_update(GameMode::Playing)
                    .with_system(animate_system)
                    .label("animate")
                    .after("player"),
            );
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
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            forces: RigidBodyForces {
                gravity_scale: 1.,
                ..Default::default()
            }
            .into(),
            damping: RigidBodyDamping {
                linear_damping: 0.5,
                ..Default::default()
            }
            .into(),
            mass_properties: RigidBodyMassProps {
                flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            collider_type: ColliderType::Solid.into(),
            shape: ColliderShape::cuboid(0.16, 0.16).into(),
            mass_properties: ColliderMassProps::Density(2.0).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Player {
            state: State::Gliding,
            direction: Direction::Right,
        })
        .insert(Timer::from_seconds(0.1, true));
}

// Manages the players velocity and position based on user input.
fn player_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut RigidBodyForcesComponent)>,
) {
    let (mut player, mut force) = query.single_mut();

    if keyboard.pressed(KeyCode::Up) {
        force.force += vector![0., FLAP_Y_FORCE];
        player.state = State::Flapping;
    }

    if keyboard.pressed(KeyCode::Left) {
        force.force += vector![-FLAP_X_FORCE, 0.];
        player.state = State::Flapping;
        player.direction = Direction::Left;
    }

    if keyboard.pressed(KeyCode::Right) {
        force.force += vector![FLAP_X_FORCE, 0.];
        player.state = State::Flapping;
        player.direction = Direction::Right;
    }

    if !keyboard.any_pressed(vec![KeyCode::Up, KeyCode::Left, KeyCode::Right])
        && player.state != State::Idle
        && player.state != State::Walking
    {
        player.state = State::Gliding;
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
                State::Walking => {
                    sprite.index += 1;
                    if sprite.index >= 3 {
                        sprite.index = 0;
                    }
                }
            }
        }
    }
}
