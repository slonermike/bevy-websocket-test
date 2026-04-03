mod plugins;

use avian2d::prelude::*;
use bevy::prelude::*;
use plugins::ball::BallPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Websocket Balls".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_walls))
        .add_plugins(BallPlugin)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::new(0.0, -500.0)))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_walls(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single().unwrap();
    let width = window.width();
    let height = window.height();

    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(width, 20.0),
        Transform::from_xyz(0.0, height * -0.5, 0.0),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(20.0, height),
        Transform::from_xyz(width * -0.5, 0.0, 0.0),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(20.0, height),
        Transform::from_xyz(width * 0.5, 0.0, 0.0),
    ));
}
