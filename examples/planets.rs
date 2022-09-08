use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use space_golf::planet::PlanetPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Planet Examples".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PlanetPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup() {}
