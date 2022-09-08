use bevy::prelude::*;
use particular::{prelude::*, ParticleSet};

pub mod golf_ball;
pub mod planet;

#[derive(Particle)]
pub struct Body {
    pub position: Vec3,
    pub mu: f32,
    pub entity: Entity,
}
impl Body {
    pub fn new(
        position: Vec3,
        mu: f32,
        entity: Entity,
    ) -> Self {
        Self {
            position,
            mu,
            entity,
        }
    }
}
