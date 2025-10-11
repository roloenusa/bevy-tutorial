use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::{Collider, RigidBody};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

mod platforms;
mod player;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const FLOOR_THICKNESS: f32 = 10.0;

const COLOR_BACKGROUND: Color = Color::linear_rgb(0.13, 0.13, 0.23);
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
        .add_plugins(platforms::PlatformsPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
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

}
