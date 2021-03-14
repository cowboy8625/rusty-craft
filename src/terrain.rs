use bevy::prelude::*;
use crate::{STATE, GameState};

pub struct Terrain;

impl Plugin for Terrain {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(STATE, GameState::InGame, setup.system());
    }
}

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
        });

    // Boxes
    let box_mesh = meshes.add(Mesh::from(shape::Cube { size: 1. }));
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
