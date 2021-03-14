mod flycam;
mod fpshud;
mod game;
mod menu;
mod terrain;

use bevy::prelude::*;

use game::GameBundlePlugins;

const STATE: &'static str = "state";
const FIRESANS_BOLD: &'static str = "../assets/font/FiraSans-Bold.ttf";
const MINECRAFT_FONT: &'static str = "../assets/font/Minecraftia-Regular.ttf";

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Rusty Craft".to_string(),
            // width: 500.,
            // height: 300.,
            // vsync: true,
            resizable: true,
            cursor_visible: true,
            // cursor_locked: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<menu::ButtonMaterials>()
        .add_resource(State::new(GameState::MainMenu))
        .add_stage_before(stage::UPDATE, STATE, StateStage::<GameState>::default())
        .add_plugins(GameBundlePlugins)
        .run();
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum GameState {
    MainMenu,
    InGame,
}
