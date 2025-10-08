use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::{Collider, RigidBody};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const FLOOR_THICKNESS: f32 = 10.0;

const COLOR_BACKGROUND: Color = Color::linear_rgb(0.29, 0.31, 0.41);
const COLOR_PLATFORM: Color = Color::linear_rgb(0.13, 0.13, 0.23);
const COLOR_PLAYER: Color = Color::linear_rgb(0.60, 0.55, 0.60);
const COLOR_FLOOR: Color = Color::linear_rgb(0.45, 0.55, 0.66);

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0)) // Physics plugin
        .add_plugins(RapierDebugRenderPlugin::default()) // Debug plugin
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: COLOR_FLOOR,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (FLOOR_THICKNESS / 2.0), 0.0),
            scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
            ..default()
        }
    ))
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.5, 0.5));

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(COLOR_PLAYER))),
        Transform {
            translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
            scale: Vec3::new(30.0, 30.0, 1.0),
            ..default()
        },
    ));

    commands.spawn(PlatformBundle::new(-100.0, Vec3::new(75.0, 200.0, 1.0)));

    commands.spawn(PlatformBundle::new(100.0, Vec3::new(50.0, 350.0, 1.0)));

    commands.spawn(PlatformBundle::new(350.0, Vec3::new(150.0, 250.0, 1.0)));
}


#[derive(Bundle)]
struct PlatformBundle {
    sprite: Sprite,
    transform: Transform,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    fn new(x: f32, scale: Vec3) -> Self {
        Self {
            sprite: Sprite {
                color: COLOR_PLATFORM,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(x, WINDOW_BOTTOM_Y + (scale.y / 2.0), 0.0),
                scale,
                ..default()
            },
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(0.5, 0.5)
        }
    }
}
