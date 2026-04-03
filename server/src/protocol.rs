use bevy::{ecs::message::Message, math::Vec2};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SpawnRequest {
    pub x: f32,
    pub y: f32,
}

#[derive(Message)]
pub struct SpawnBallMessage {
    pub position: Vec2,
    pub velocity: Vec2,
}
