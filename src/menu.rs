use crate::{GameState, STATE, MINECRAFT_FONT, FIRESANS_BOLD};
use bevy::prelude::*;

enum ButtonType {
    Start,
    Exit,
}

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(STATE, GameState::MainMenu, spawn_main_menu.system())
            .on_state_update(STATE, GameState::MainMenu, menu_buttons.system())
            .on_state_exit(STATE, GameState::MainMenu, close_menu.system());
    }
}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn spawn_main_menu(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        // ui camera
        .spawn(CameraUiBundle::default())
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: "Start".to_string(),
                    font: asset_server.load(MINECRAFT_FONT),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        })
        .with(ButtonType::Start)
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 70.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: "Exit".to_string(),
                    font: asset_server.load(MINECRAFT_FONT),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        })
        .with(ButtonType::Exit);
}

fn menu_buttons(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Handle<ColorMaterial>, /*, &Children*/
            &ButtonType,
        ),
        (Mutated<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<State<GameState>>,
    mut exit: ResMut<Events<bevy::app::AppExit>>,
) {
    for (interaction, mut material/*, children*/, button_type) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                match button_type {
                    ButtonType::Start => app_state.set_next(GameState::InGame).unwrap(),
                    ButtonType::Exit => exit.send(bevy::app::AppExit),
                }
            },
            Interaction::Hovered => {
                // text.value = "Hover".to_string();
                *material = button_materials.hovered.clone();
            },
            Interaction::None => {
                // text.value = "Button".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn close_menu(command: &mut Commands, query: Query<(Entity, &Button)>) {
    for (e, _) in query.iter() {
        command.despawn_recursive(e);
    }
}
