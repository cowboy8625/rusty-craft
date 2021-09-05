use bevy::{
    prelude::*,
    render::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
};
use bevy_flycam::{NoCameraPlayerPlugin, FlyCam};
use bevy_mod_picking::{
    HighlightablePickingPlugin,
    InteractablePickingPlugin, PickableBundle, PickingCameraBundle, PickingPlugin,
};

// Components
struct Player;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuOptions {
            features: WgpuFeatures {
                // The Wireframe requires NonFillPolygonMode feature
                features: vec![WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(PickingPlugin) // InteractablePickingPlugin adds mouse events and selection
        .add_plugin(InteractablePickingPlugin) // HighlightablePickingPlugin adds hover, click, and selection highlighting
        .add_plugin(HighlightablePickingPlugin) // DebugPickingPlugin systems to build and update debug cursors
        .add_startup_system(setup.system())
        .add_startup_system(player_spawn.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // To draw the wireframe on all entities, set this to 'true'
    wireframe_config.global = false;
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    })
    .insert_bundle(PickableBundle::default());
    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        // This enables wireframe drawing on this entity
        .insert(Wireframe)
        .insert_bundle(PickableBundle::default());
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn player_spawn(mut commands: Commands) {
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Player)
        .insert_bundle(PickingCameraBundle::default())
        .insert(FlyCam);
}