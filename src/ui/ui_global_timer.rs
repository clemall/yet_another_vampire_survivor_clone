use crate::components::*;
use crate::constants::{
    DAMAGE_FONT, DAMAGE_FONT_COLOR, DAMAGE_FONT_SIZE, FONT_BOLD, MAP_LEVEL_EXPERIENCE,
};
use bevy::prelude::*;

pub struct UiGlobalTimerPlugin;

impl Plugin for UiGlobalTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_global_timer_ui);
        app.add_systems(
            Update,
            (global_timer_ui_sync).run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn setup_global_timer_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let parent_node = commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Px(20.),
                    top: Val::Px(20.),
                    // align_items: AlignItems::Center,
                    // justify_content: JustifyContent::Center,
                    // position_type: PositionType::Absolute,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("Global timer UI "),
        ))
        .id();

    let timer_text = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        &format!("00:00"),
                        TextStyle {
                            font: asset_server.load(DAMAGE_FONT),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    )],
                    ..default()
                },
                z_index: ZIndex::Local(1),
                ..default()
            },
            GlobalTimerUI,
        ))
        .id();

    let timer_text_shadow = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        &format!("00:00"),
                        TextStyle {
                            font: asset_server.load(DAMAGE_FONT),
                            font_size: 24.0,
                            color: Color::BLACK,
                        },
                    )],
                    ..default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(2.0),
                    left: Val::Px(2.0),
                    ..default()
                },
                z_index: ZIndex::Local(-1),
                ..default()
            },
            GlobalTimerUI,
        ))
        .id();

    commands
        .entity(parent_node)
        .push_children(&[timer_text, timer_text_shadow]);
}

fn global_timer_ui_sync(
    mut ui: Query<&mut Text, With<GlobalTimerUI>>,
    global_timer: Res<WaveManagerGlobalTime>,
) {
    let total_seconds = global_timer.global_time.elapsed().as_secs();

    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    for mut text in &mut ui {
        text.sections[0].value = format!("{minutes:0>2}:{seconds:0>2}");
    }
}
