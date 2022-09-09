use bevy::{
    math::Vec3Swizzles,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::{
            AsBindGroup, PrimitiveTopology, ShaderRef,
        },
    },
};

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(
            MaterialPlugin::<Planet3dMaterial>::default(),
        )
        .add_plugin(WireframePlugin)
        .add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Planet3dMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands
        .spawn()
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(PlanetMesh {
                resolution: 10,
            })),
            transform: Transform::from_xyz(-2.0, 0.5, 0.0),
            material: materials.add(Planet3dMaterial {
                color: Color::RED,
            }),
            ..default()
        })
        .insert(Wireframe);
    // commands
    //     .spawn()
    //     .insert_bundle(MaterialMeshBundle {
    //         mesh: meshes.add(Mesh::from(PlanetMesh {
    //             resolution: 20,
    //         })),
    //         transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //         material: materials.add(Planet3dMaterial {
    //             color: Color::RED,
    //         }),
    //         ..default()
    //     })
    //     .insert(Wireframe);
    // commands
    //     .spawn()
    //     .insert_bundle(MaterialMeshBundle {
    //         mesh: meshes.add(Mesh::from(PlanetMesh {
    //             resolution: 40,
    //         })),
    //         transform: Transform::from_xyz(2.0, 0.5, 0.0),
    //         material: materials.add(Planet3dMaterial {
    //             color: Color::RED,
    //         }),
    //         ..default()
    //     })
    //     .insert(Wireframe);
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for Planet3dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet_3d_material.wgsl".into()
    }

    // fn alpha_mode(&self) -> AlphaMode {
    //     self.alpha_mode
    // }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45db-8225-97e2a3f056e0"]
pub struct Planet3dMaterial {
    #[uniform(0)]
    color: Color,
}

struct PlanetMesh {
    resolution: u32,
}

impl From<PlanetMesh> for Mesh {
    fn from(planet: PlanetMesh) -> Self {
        let directions = [
            Vec3::Y,
            Vec3::NEG_Y,
            Vec3::NEG_X,
            Vec3::X,
            Vec3::Z,
            Vec3::NEG_Z,
        ];

        let (vert_lists, triangle_lists): (
            Vec<Vec<Vec3>>,
            Vec<Vec<u32>>,
        ) = directions
            .iter()
            .map(|direction| {
                let t = face(planet.resolution, *direction);
                dbg!(&t.1.len());
                t
            })
            .unzip();

        let vertices = vert_lists
            .iter()
            .flat_map(|v| v.iter().map(|v| [v.x, v.y, v.z]))
            .collect::<Vec<[f32; 3]>>();

        let triangle_list = triangle_lists
            .iter()
            .enumerate()
            .flat_map(|(face_id, list)| {
                // local_face_index indexes go up to resolution^2 - 1.
                // so the last vertex in a face with a resolution of
                // 10 is index 99 (100 indices, starting at 0).
                //
                // that makes the *index* of the second face's vertices
                // start at 100 and end at 199.
                list.iter().map(move |local_idx| {
                    let num_indices = planet.resolution
                        * planet.resolution;
                    local_idx + face_id as u32 * num_indices
                })
            })
            .collect::<Vec<u32>>();

        let mut mesh =
            Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(
            triangle_list.clone(),
        )));
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices.clone(),
        );

        // unit sphere means normals are already calculated
        // because a vertex on a unit sphere is a vector from
        // the center
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vertices.clone(),
        );
        // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        // Insert the vertex colors as an attribute
        if let Some(VertexAttributeValues::Float32x3(
            positions,
        )) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[r, g, b]| {
                    [
                        (1. - *r) / 2.,
                        (1. - *g) / 2.,
                        (1. - *b) / 2.,
                        1.,
                    ]
                })
                .collect();
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            );
        };
        mesh
    }
}

/// build one face of the "cubesphere"
/// resolution is the per-face resolution,
/// the number of lines, which in turns means
/// resolution-1 squares per axis on each face
fn face(
    resolution: u32,
    local_up: Vec3,
) -> (Vec<Vec3>, Vec<u32>) {
    let axis_a = local_up.yzx();
    let axis_b = local_up.cross(axis_a);

    let mut vertices = Vec::with_capacity(
        resolution as usize * resolution as usize,
    );

    // a resolution of 10 means 10 lines
    // which is 9 squares per side,
    // with 2 triangles per square
    // 3 vertices per triangle
    let mut triangles = Vec::with_capacity(
        (resolution as usize - 1)
            * (resolution as usize - 1)
            * 6,
    );

    for y in 0..resolution {
        for x in 0..resolution {
            let i = x + y * resolution;
            let percent_x =
                x as f32 / (resolution - 1) as f32;
            let percent_y =
                y as f32 / (resolution - 1) as f32;

            let point_on_unit_cube = local_up
                + (percent_x - 0.5) * 2.0 * axis_a
                + (percent_y - 0.5) * 2.0 * axis_b;
            let point_on_unit_sphere =
                point_on_unit_cube.normalize();

            vertices.push(point_on_unit_sphere);

            if x != resolution - 1 && y != resolution - 1 {
                // triangle list vertices 1
                triangles.push(i);
                triangles.push(i + resolution + 1);
                triangles.push(i + resolution);

                // triangle list vertices 2
                triangles.push(i);
                triangles.push(i + 1);
                triangles.push(i + resolution + 1);
            }
        }
    }
    (vertices, triangles)
}
