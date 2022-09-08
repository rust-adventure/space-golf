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
                resolution: 20,
            })),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: materials.add(Planet3dMaterial {
                color: Color::RED,
            }),
            ..default()
        })
        .insert(Wireframe);

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

        let verts_and_triangles = directions
            .iter()
            .map(|direction| {
                face(planet.resolution, *direction)
            })
            .collect::<Vec<(Vec<Vec3>, Vec<u32>)>>();
        let vertices = &verts_and_triangles
            .iter()
            .flat_map(|(verts, _)| {
                verts.iter().map(|v| [v.x, v.y, v.z])
            })
            .collect::<Vec<[f32; 3]>>();
        let indices = &verts_and_triangles
            .iter()
            .enumerate()
            .flat_map(|(enum_index, (_, idxs))| {
                idxs.iter()
                    .map(|i| {
                        *i + enum_index as u32
                            * (planet.resolution
                                * planet.resolution)
                    })
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<u32>>();

        let mut mesh =
            Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(
            indices.clone(),
        )));
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices.clone(),
        );

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vertices
                .iter()
                .map(|[x, y, z]| [*x, *y, *z])
                .collect::<Vec<[f32; 3]>>(),
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

fn face(
    resolution: u32,
    local_up: Vec3,
) -> (Vec<Vec3>, Vec<u32>) {
    let axis_a = local_up.yzx();
    let axis_b = local_up.cross(axis_a);

    let mut vertices: Vec<Vec3> = (0..(resolution
        * resolution))
        .into_iter()
        .map(|_| Vec3::ZERO)
        .collect();

    let mut triangles: Vec<u32> =
        (0..((resolution - 1) * (resolution - 1) * 6))
            .into_iter()
            .map(|_| 0)
            .collect();

    let mut tri_index: usize = 0;

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

            vertices[i as usize] = point_on_unit_sphere;

            if x != resolution - 1 && y != resolution - 1 {
                triangles[tri_index] = i;
                triangles[tri_index + 1] =
                    i + resolution + 1;
                triangles[tri_index + 2] = i + resolution;

                triangles[tri_index + 3] = i;
                triangles[tri_index + 4] = i + 1;
                triangles[tri_index + 5] =
                    i + resolution + 1;
                tri_index += 6;
            }
        }
    }
    (vertices, triangles)
}
