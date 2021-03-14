use crate::{
    flycam::{FlyCamera, FlyCameraPlugin},
    fpshud::FPSPlugin,
    menu::{ButtonMaterials, Menu},
    terrain::Terrain,
    GameState, STATE,
};
use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;

pub struct GameBundlePlugins;

impl PluginGroup for GameBundlePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(Terrain)
            .add(FPSPlugin)
            .add(FlyCameraPlugin)
            .add(Menu);
    }
}

impl Plugin for GameBundlePlugins {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(Msaa { samples: 4 })
            .init_resource::<ButtonMaterials>()
            .add_plugins(Self)
            .on_state_update(STATE, GameState::InGame, toggle_cursor.system());
    }
}

pub enum Focus {
    Menu,
    Game,
}

pub struct PlayerSettings {
    pub focus: Focus,
}


fn toggle_cursor(
    input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut query: Query<&mut PlayerSettings, With<FlyCamera>>,
) {
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
