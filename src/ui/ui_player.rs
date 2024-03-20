use bevy::prelude::*;
use crate::components::*;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};


pub struct UiPlayerPlugin;

impl Plugin for UiPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ( setup_player_ui));
        app.add_systems(Update, (
            player_health_ui_sync
            )
        );
    }
}


fn setup_player_ui(mut commands: Commands,
         asset_server: Res<AssetServer>,
) {

    let parent_node = (
        NodeBundle {
            style: Style {
                width: Val::Px(80.),
                height: Val::Px(5.),
                // WTF, should be SCREEN_WIDTH / 2... but the screen UI seems to be 1200px,
                left: Val::Px(SCREEN_WIDTH as f32 - 40.0),
                right: Val::Auto,
                top: Val::Px(SCREEN_HEIGHT as f32 + 20.0),
                bottom: Val::Auto,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        PlayerUI,
        Name::new("Player UI"),
    );

    let health_node = (
        NodeBundle {
            style: Style {
                width: Val::Px(80.),
                height: Val::Px(5.),
                ..default()
            },
            background_color: BackgroundColor(Color::RED),
            ..default()
        },
        HealthUI,
        Name::new("Health UI"),
    );

    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(health_node);
    });

}


fn player_health_ui_sync(mut ui: Query<&mut Style, With<HealthUI>>, health: Query<(&Health,&MaxHealth), With<Player>>) {
    let mut style = ui.single_mut();
    let (health, max_health) = health.single();

    let percent = health.0 / max_health.0;
    style.width = Val::Percent(percent * 100.0);
}
