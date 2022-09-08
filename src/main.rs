use std::f32::consts::PI;

use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    math::Vec3Swizzles,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mouse_tracking_plugin::{
    MousePosPlugin, MousePosWorld,
};
use bevy_rapier2d::{
    prelude::*, render::RapierDebugRenderPlugin,
};
use particular::ParticleSet;
use space_golf::{
    golf_ball::{
        CircleWithGravity, GolfBallSettings, PointMass,
    },
    planet::PlanetPlugin,
    Body,
};

const G: f32 = 1000.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Space Golf!".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .init_resource::<GolfBallSettings>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(ParticleSet::<Body>::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(MousePosPlugin::SingleCamera)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(PlanetPlugin)
        .add_startup_system(setup)
        .add_system(place_body)
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(sync_particle_set),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_system(accelerate_particles),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;
    commands.spawn_bundle(Camera2dBundle::default());

    /// Planet 1
    let mass = 10E5;
    let density = 20.0;
    let radius = (mass / (density * PI)).sqrt();
    let entity = commands.spawn_bundle(CircleWithGravity {
        shape_bundle: MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Circle {
                    radius,
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                -300.0, -50.0, 1.0,
            ),
            material: materials
                .add(ColorMaterial::from(Color::BLUE)),
            ..default()
        },
        collider: Collider::ball(radius),
        friction: Friction {
            coefficient: 10.0,
            ..default()
        },
        mass: ColliderMassProperties::Mass(mass),
        restitution: Restitution {
            coefficient: 0.0,
            ..default()
        },
        rigidbody: RigidBody::Fixed,
        velocity: Velocity::zero(),
        acceleration: ExternalForce::default(),
        point_mass: PointMass::HasGravity {
            // mass: body_info.mass,
            mass: mass,
        },
    });

    /// Planet 2
    let mass = 8E5;
    let density = 20.0;
    let radius = (mass / (density * PI)).sqrt();
    let entity = commands.spawn_bundle(CircleWithGravity {
        shape_bundle: MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Circle {
                    radius,
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                200.0, 10.0, 1.0,
            ),
            material: materials
                .add(ColorMaterial::from(Color::BLUE)),
            ..default()
        },
        collider: Collider::ball(radius),
        friction: Friction {
            coefficient: 10.0,
            ..default()
        },
        mass: ColliderMassProperties::Mass(mass),
        restitution: Restitution {
            coefficient: 0.0,
            ..default()
        },
        rigidbody: RigidBody::Fixed,
        velocity: Velocity::zero(),
        acceleration: ExternalForce::default(),
        point_mass: PointMass::HasGravity {
            // mass: body_info.mass,
            mass: mass,
        },
    });
}

fn sync_particle_set(
    mut particle_set: ResMut<ParticleSet<Body>>,
    query: Query<(Entity, &GlobalTransform, &PointMass)>,
) {
    *particle_set = ParticleSet::new();
    query.for_each(|(entity, tranform, point_mass)| {
        let mu = match point_mass {
            PointMass::HasGravity { mass } => *mass * G,
            PointMass::AffectedByGravity => 0.0,
        };
        particle_set.add(Body::new(
            tranform.translation(),
            mu,
            entity,
        ));
    })
}

fn accelerate_particles(
    mut particle_set: ResMut<ParticleSet<Body>>,
    mut query: Query<&mut ExternalForce, With<PointMass>>,
) {
    for (body, gravity) in particle_set.result() {
        if let Ok(mut acceleration) =
            query.get_mut(body.entity)
        {
            acceleration.force = gravity.xy();
        }
    }
}

fn place_body(
    mut commands: Commands,
    mut click_event: EventReader<MouseButtonInput>,
    // mut lines: ResMut<DebugLines>,
    mut body_info: ResMut<GolfBallSettings>,
    mouse_pos: Res<MousePosWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mouse_pos = mouse_pos.truncate().extend(0.0);

    for event in click_event.iter() {
        if event.button == MouseButton::Left {
            match event.state {
                ButtonState::Pressed => {
                    body_info.position = Some(mouse_pos)
                }
                ButtonState::Released => {
                    if let Some(place_pos) =
                        body_info.position.take()
                    {
                        let mass: f32 = 100.0_f32.max(1.0);
                        let density = 1.0;
                        let radius =
                            (mass / (density * PI)).sqrt();
                        let entity = commands.spawn_bundle(CircleWithGravity {
                            shape_bundle: MaterialMesh2dBundle {
                                mesh: meshes
                                    .add(Mesh::from(shape::Circle {
                                        radius,
                                        ..default()
                                    }))
                                    .into(),
                                transform: Transform::from_xyz(place_pos.x, place_pos.y, place_pos.z),
                                material: materials
                                    .add(ColorMaterial::from(Color::WHITE)),
                                ..default()
                            },
                            collider: Collider::ball(radius),
                            friction: Friction {
                                coefficient: 10.0,
                                ..default()
                            },
                            mass: ColliderMassProperties::Mass(mass),
                            restitution: Restitution {
                                coefficient: 0.0,
                                ..default()
                            },
                            rigidbody: RigidBody::Dynamic,
                            velocity: Velocity::linear((place_pos - mouse_pos).xy()),
                            acceleration: ExternalForce::default(),
                            point_mass: PointMass::HasGravity {
                                // mass: body_info.mass,
                                mass: mass,
                            },
                        });

                        // if body_info.trail {
                        //     entity.insert(Trail::new(
                        //         20.0, 1,
                        //     ));
                        // }
                    }
                }
            }
        }
    }

    // if let Some(place_pos) = body_info.position {
    //     let scale = (mouse_pos
    //         .distance_squared(place_pos)
    //         .powf(0.04)
    //         - 1.0)
    //         .clamp(0.0, 1.0);
    //     lines.line_colored(
    //         place_pos,
    //         mouse_pos,
    //         0.0,
    //         Color::rgb(scale, 1.0 - scale, 0.0),
    //     )
    // }
}
