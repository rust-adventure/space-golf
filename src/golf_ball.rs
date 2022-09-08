use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use bevy_rapier2d::prelude::*;
use particular::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub enum PointMass {
    HasGravity { mass: f32 },
    AffectedByGravity,
}

#[derive(Bundle)]
pub struct CircleWithGravity<M: Material2d> {
    #[bundle]
    pub shape_bundle: MaterialMesh2dBundle<M>,
    pub collider: Collider,
    pub friction: Friction,
    pub mass: ColliderMassProperties,
    pub restitution: Restitution,
    pub rigidbody: RigidBody,
    pub velocity: Velocity,
    pub acceleration: ExternalForce,
    pub point_mass: PointMass,
}

pub struct GolfBallSettings {
    pub position: Option<Vec3>,
    pub mass: f32,
    pub trail: bool,
}

impl Default for GolfBallSettings {
    fn default() -> Self {
        Self {
            position: None,
            mass: 20.0,
            trail: false,
        }
    }
}
