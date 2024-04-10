use crate::components::*;
use bevy::prelude::*;

pub struct UiLevelUpPlugin;

impl Plugin for UiLevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PlayerLevelUp), spawn_level_up_ui);
        app.add_systems(OnExit(GameState::PlayerLevelUp), despawn_level_up_ui);
        app.add_systems(
            Update,
            button_interaction.run_if(in_state(GameState::PlayerLevelUp)),
        );
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn despawn_level_up_ui(mut commands: Commands, ui: Query<Entity, With<LevelUpUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}

fn spawn_level_up_ui(mut commands: Commands) {
    let level_up_parent = (
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
        LevelUpUI,
        Name::new("UI Group Level UP"),
    );

    let level_up_popup = NodeBundle {
        style: Style {
            width: Val::Percent(50.0),
            height: Val::Percent(50.0),
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        },
        background_color: Color::DARK_GRAY.into(),
        ..default()
    };

    let button_test = (
        ButtonBundle {
            style: Style {
                width: Val::Px(150.),
                height: Val::Px(65.),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            // image: UiImage::default().with_color( NORMAL_BUTTON),
            ..default()
        },
        ButtonLevelUpUI,
    );

    let button_test_text = TextBundle::from_section(
        "Move on for now",
        TextStyle {
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    commands.spawn(level_up_parent).with_children(|commands| {
        commands.spawn(level_up_popup).with_children(|commands| {
            commands.spawn(button_test).with_children(|commands| {
                commands.spawn(button_test_text);
            });
        });
    });
}

fn button_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor), // UiImage
        (Changed<Interaction>, With<Button>, With<ButtonLevelUpUI>),
    >,
) {
    for (interaction, mut image) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *image = PRESSED_BUTTON.into();
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
