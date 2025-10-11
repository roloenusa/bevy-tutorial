use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::WINDOW_BOTTOM_Y;

const COLOR_PLATFORM: Color = Color::linear_rgb(0.29, 0.31, 0.41);

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}


fn setup(mut commands: Commands) {
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
