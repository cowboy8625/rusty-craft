use crate::{GameState, STATE};
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct FPSPlugin;
impl Plugin for FPSPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .on_state_enter(STATE, GameState::InGame, fps_setup.system())
            .on_state_update(STATE, GameState::InGame, fps_update.system());
    }
}

struct FpsText;

fn fps_update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

fn fps_setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        // 2d camera
        .spawn(CameraUiBundle::default())
        // texture
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: asset_server.load("../assets/font/Minecraftia-Regular.ttf"),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::BLUE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(FpsText);
}
