use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::WINDOW_BOTTOM_Y;
use crate::WINDOW_LEFT_X;

const PLAYER_VELOCITY_X: f32 = 400.0;
const PLAYER_VELOCITY_Y: f32 = 850.0;

const MAX_JUMP_HEIGHT: f32 = 230.0;

const SPRITESHEET_COLS: u32 = 7;
const SPRITESHEET_ROWS: u32 = 8;

const SPRITE_TILE_WIDTH: u32 = 128;
const SPRITE_TILE_HEIGHT: u32 = 256;

const SPRITE_IDX_STAND: usize = 28;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (movement, jump, rise, fall));
    }
}

fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    server: Res<AssetServer>,
) {
    let texture: Handle<Image> = server.load("spritesheets/spritesheet_players.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        None,
        None
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: SPRITE_IDX_STAND
            }),
            ..default()
        },
        Transform {
            translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
            ..default()
        }
    ))
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::ball(0.5))
    .insert(KinematicCharacterController::default());
}

#[derive(Component)]
struct Jump(f32);

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<&mut KinematicCharacterController>,
) {
    let mut player = player.into_inner();
    let mut movement = 0.0;

    if input.pressed(KeyCode::ArrowRight) {
        movement = time.delta_secs() * PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::ArrowLeft) {
        movement = -(time.delta_secs() * PLAYER_VELOCITY_X);
    }


    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)),
        None => player.translation = Some(Vec2::new(movement, 0.0))
    }
}

fn jump(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    single: Single<(Entity, &KinematicCharacterControllerOutput), (With<KinematicCharacterController>, Without<Jump>)>
) {
    let player = single.into_inner();

    let (player, output) = player;

    if input.pressed(KeyCode::ArrowUp) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
    }
}

fn rise(
    mut commands: Commands,
    time: Res<Time>,
    player: Single<(Entity, &mut KinematicCharacterController, &mut Jump)>
) {
    let player = player.into_inner();

    let (entity, mut player, mut jump) = player;

    let mut movement = time.delta_secs() * PLAYER_VELOCITY_Y;

    if movement + jump.0 >= MAX_JUMP_HEIGHT {
        movement = MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

fn fall(
    time: Res<Time>,
    player: Single<&mut KinematicCharacterController, Without<Jump>>
) {
    let mut player = player.into_inner();

    let movement = -(time.delta_secs() * PLAYER_VELOCITY_Y / 1.5);

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}
