use crate::cameras::camera::setup_camera;
use crate::components::*;
use crate::constants::{MAP_LEVEL_EXPERIENCE, SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::prelude::*;

pub struct UiPlayerPlugin;

impl Plugin for UiPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (setup_player_health_ui, setup_player_experience_bar_ui),
        );
        app.add_systems(
            Update,
            (player_health_ui_sync, player_experience_bar_ui_sync),
        );
    }
}

fn setup_player_health_ui(mut commands: Commands) {
    let parent_node = commands
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
            Name::new("Player UI "),
        ))
        .id();

    let health_container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(8.0),
                    margin: UiRect::top(Val::Px(65.0)),
                    ..default()
                },
                ..default()
            },
            PlayerHealthUIParent,
            Name::new("Health UI background"),
        ))
        .id();

    let health_background = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            PlayerHealthUIParent,
            Name::new("Health UI background"),
        ))
        .id();

    let health_front = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::RED),
                ..default()
            },
            PlayerHealthUI,
            Name::new("Health UI"),
        ))
        .id();

    commands
        .entity(parent_node)
        .push_children(&[health_container]);

    commands
        .entity(health_container)
        .push_children(&[health_background]);

    commands
        .entity(health_background)
        .push_children(&[health_front]);
}

fn player_health_ui_sync(
    mut ui: Query<&mut Style, With<PlayerHealthUI>>,
    health: Query<(&Health, &MaxHealth), With<Player>>,
) {
    let mut style = ui.single_mut();
    let (health, max_health) = health.single();

    let percent = health.0 / max_health.0;
    style.width = Val::Percent(percent * 100.0);
}

fn setup_player_experience_bar_ui(mut commands: Commands) {
    let parent_node = (
        NodeBundle {
            style: Style {
                width: Val::Vw(80.0),
                height: Val::Px(20.),
                left: Val::Vw(10.0),
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Px(20.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        },
        PlayerExperienceBarUIParent,
        Name::new("Player experience bar UI"),
    );

    let bar_node = (
        NodeBundle {
            style: Style {
                width: Val::Px(10.),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                ..default()
            },
            background_color: BackgroundColor(Color::BLUE),
            ..default()
        },
        PlayerExperienceUI,
        Name::new("Player exeprience UI"),
    );

    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(bar_node);
    });
}

fn player_experience_bar_ui_sync(
    mut ui: Query<&mut Style, With<PlayerExperienceUI>>,
    player_experience: Res<PlayerExperience>,
) {
    let mut style = ui.single_mut();

    let total = MAP_LEVEL_EXPERIENCE[player_experience.level as usize];

    //early return when the value is 0 to avoid dividing by 0
    if player_experience.amount_experience == 0 {
        style.width = Val::Percent(0.0);
        return;
    }

    let percent = player_experience.amount_experience as f32 / total as f32;
    style.width = Val::Percent(percent * 100.0);
}
