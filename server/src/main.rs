use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Websocket Balls".into(),
                ..default()
            }),
            ..default()
        }))
        .add_message::<SpawnBallMessage>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, spawn_ball)
        .add_systems(Startup, send_test_spawn)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Message)]
struct SpawnBallMessage {
    position: Vec2,
}

fn spawn_ball(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut messages: MessageReader<SpawnBallMessage>,
) {
    for message in messages.read() {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(25.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
            Transform::from_translation(message.position.extend(0.0)),
        ));
    }
}

fn send_test_spawn(mut writer: MessageWriter<SpawnBallMessage>) {
    writer.write(SpawnBallMessage {
        position: Vec2::new(0.0, 0.0),
    });
}
