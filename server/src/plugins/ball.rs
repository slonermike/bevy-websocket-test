use avian2d::prelude::*;
use bevy::prelude::*;
use rand::RngExt;

use crate::protocol::SpawnBallMessage;

pub struct BallPlugin;

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawn_ball_handler(
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
            RigidBody::Dynamic,
            Collider::circle(25.0),
            Restitution::new(1.0),
            LinearVelocity(message.velocity),
        ));
    }
}

fn spawn_timer_system(
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    mut writer: MessageWriter<SpawnBallMessage>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let mut rng = rand::rng();

        let rand_x = rng.random_range(-150.0..150.0);
        let rand_y = rng.random_range(-150.0..150.0);
        writer.write(SpawnBallMessage {
            position: Vec2::new(rand_x, rand_y),
            velocity: Vec2::new(rand_x, rand_y),
        });

        let next_duration = rng.random_range(0.5..1.5);
        timer
            .0
            .set_duration(std::time::Duration::from_secs_f32(next_duration));
    }
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnBallMessage>()
            .insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_systems(Update, (spawn_timer_system, spawn_ball_handler));
    }
}
