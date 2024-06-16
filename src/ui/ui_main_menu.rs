use crate::components::*;
use crate::constants::{FONT_BOLD, SCREEN_WIDTH};
use bevy::prelude::*;
use rand::prelude::SliceRandom;

pub struct UiMainMenuPlugin;

impl Plugin for UiMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu_ui);
        app.add_systems(OnExit(GameState::MainMenu), despawn_main_menu_ui);
        app.add_systems(
            Update,
            update_main_menu_button_interaction.run_if(in_state(GameState::MainMenu)),
        );
    }
}

const HOVERED_BUTTON: Color = Color::rgb(0.0, 0.80, 0.80);
const NORMAL_BUTTON: Color = Color::rgb(0., 1., 1.);
const POPUP_BG_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.95);

fn despawn_main_menu_ui(mut commands: Commands, ui: Query<Entity, With<MainMenuUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}

fn spawn_main_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_parent = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MainMenuUI,
            Name::new("UI main menu UP"),
        ))
        .id();

    let main_menu_popup = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                position_type: PositionType::Relative,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            background_color: POPUP_BG_COLOR.into(),
            ..default()
        })
        .id();

    let play_button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    width: Val::Px(200.),
                    height: Val::Px(90.0),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                z_index: ZIndex::Global(10),
                ..default()
            },
            MainMenuPlayButton,
        ))
        .id();

    let play_button_label = commands
        .spawn(TextBundle::from_section(
            "Play",
            TextStyle {
                font: asset_server.load(FONT_BOLD),
                font_size: 36.0,
                color: Color::BLACK,
                ..default()
            },
        ))
        .id();

    commands
        .entity(play_button)
        .push_children(&[play_button_label]);

    commands
        .entity(main_menu_popup)
        .push_children(&[play_button]);

    commands
        .entity(main_menu_parent)
        .push_children(&[main_menu_popup]);
}

fn update_main_menu_button_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MainMenuPlayButton), // UiImage
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut image, upgrade) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Gameplay);
            }
            Interaction::Hovered => {
                *image = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *image = NORMAL_BUTTON.into();
            }
        }
    }
}
