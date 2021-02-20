mod flycam;
mod fpshud;
use bevy::prelude::*;
use flycam::{FlyCamera, FlyCameraPlugin};
use fpshud::FPSPlugin;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Rusty Craft".to_string(),
            // width: 500.,
            // height: 300.,
            vsync: true,
            resizable: true,
            cursor_visible: false,
            cursor_locked: true,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(FPSPlugin)
        .add_startup_system(setup.system())
        .add_system(toggle_cursor.system())
        .run();
}

enum Focus {
    Menu,
    Game,
}

struct PlayerSettings {
    focus: Focus,
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(FlyCamera::default()).with(PlayerSettings { focus: Focus::Game });

    // Boxes
    let box_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.5 }));
    let box_material = materials.add(Color::rgb(0.0, 0.5, 0.5).into());

    const AMOUNT: i32 = 6;
    for x in -(AMOUNT / 2)..(AMOUNT / 2) {
        for y in -(AMOUNT / 2)..(AMOUNT / 2) {
            for z in -(AMOUNT / 2)..(AMOUNT / 2) {
                commands.spawn(PbrBundle {
                    mesh: box_mesh.clone(),
                    material: box_material.clone(),
                    transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
                    ..Default::default()
                });
            }
        }
    }
}

fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>, mut query: Query<&mut PlayerSettings, With<FlyCamera>>) {
    let window = windows.get_primary_mut().unwrap();
    for mut ps in query.iter_mut() {
        if input.just_pressed(KeyCode::T) {
            window.set_cursor_lock_mode(!window.cursor_locked());
            window.set_cursor_visibility(!window.cursor_visible());
            ps.focus = match ps.focus {
                Focus::Game => Focus::Menu,
                Focus::Menu => Focus::Game,
            };
        }
    }
}

