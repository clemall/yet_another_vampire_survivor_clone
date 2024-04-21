use crate::components::*;
use crate::constants::{DAMAGE_FONT, FONT, FONT_BOLD, MAP_LEVEL_EXPERIENCE};
use bevy::prelude::*;

pub struct UiPlayerPlugin;

impl Plugin for UiPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_player_health_ui,
                setup_player_experience_bar_ui,
                setup_player_level_ui,
            ),
        );
        app.add_systems(
            Update,
            (
                player_health_ui_sync,
                player_experience_bar_ui_sync,
                player_level_ui_sync,
            ),
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
                width: Val::Vw(96.0),
                height: Val::Px(30.),
                left: Val::Vw(2.0),
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Px(20.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::ALICE_BLUE),
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
            background_color: BackgroundColor(Color::rgb(0.3137, 0.6470, 0.9215)),
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

fn setup_player_level_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let parent_node = commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Px(20.),
                    width: Val::Vw(100.0),
                    bottom: Val::Px(22.),
                    right: Val::Vw(3.0),
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::End,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Global(20),
                ..default()
            },
            Name::new("Current player Level UI "),
        ))
        .id();

    let level_text = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        &format!("Level: 1"),
                        TextStyle {
                            font: asset_server.load(FONT_BOLD),
                            font_size: 26.0,
                            color: Color::BLACK,
                        },
                    )],
                    ..default()
                },
                z_index: ZIndex::Local(1),
                ..default()
            },
            PlayerLevelUI,
        ))
        .id();

    commands.entity(parent_node).push_children(&[level_text]);
}

fn player_level_ui_sync(
    mut ui: Query<&mut Text, With<PlayerLevelUI>>,
    player_experience: Res<PlayerExperience>,
) {
    for mut text in &mut ui {
        text.sections[0].value = format!("Level:{}", player_experience.level);
    }
}
