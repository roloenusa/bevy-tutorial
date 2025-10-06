use bevy::prelude::*;
use bevy::color::palettes::css::LIMEGREEN;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: LIMEGREEN.into(),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(50.0, 100.0, 1.0),
            ..Default::default()
        },
    ));
}
