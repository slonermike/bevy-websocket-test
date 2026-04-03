mod plugins;
mod protocol;
mod websocket;

use avian2d::prelude::*;
use bevy::prelude::*;
use plugins::ball::BallPlugin;
use protocol::SpawnRequest;
use std::sync::Mutex;
use std::sync::mpsc;
use websocket::{SpawnReceiver, handle_websocket_spawns, run_server};

fn main() {
    let (tx, rx) = mpsc::channel::<SpawnRequest>();

    // Start the websocket service on a parallel thread.
    std::thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(run_server(tx))
    });

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Websocket Balls".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_walls))
        .add_systems(Update, handle_websocket_spawns)
        .add_plugins(BallPlugin)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::new(0.0, -500.0)))
        .insert_resource(SpawnReceiver(Mutex::new(rx)))
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
