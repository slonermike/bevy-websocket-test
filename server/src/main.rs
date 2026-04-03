mod plugins;

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
        .add_systems(Startup, setup_camera)
        .add_plugins(BallPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
