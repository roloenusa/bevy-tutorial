use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::WINDOW_BOTTOM_Y;
use crate::WINDOW_LEFT_X;

const COLOR_PLAYER: Color = Color::linear_rgb(0.60, 0.55, 0.60);

const PLAYER_VELOCITY_X: f32 = 400.0;
const PLAYER_VELOCITY_Y: f32 = 850.0;

const MAX_JUMP_HEIGHT: f32 = 230.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (movement, jump, rise, fall));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(COLOR_PLAYER))),
        Transform {
            translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
            scale: Vec3::new(30.0, 30.0, 1.0),
            ..default()
        },
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
