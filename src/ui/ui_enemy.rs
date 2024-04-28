use crate::components::*;
use crate::constants::{DAMAGE_FONT, DAMAGE_FONT_COLOR, DAMAGE_FONT_SIZE};
use bevy::prelude::*;

pub struct UiEnemyPlugin;

impl Plugin for UiEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (enemy_received_damage_ui, update_enemy_received_damage_ui),
        );
    }
}

pub fn enemy_received_damage_ui(
    mut commands: Commands,
    mut enemies: Query<&Transform, With<Enemy>>,
    mut eneny_hit_event: EventReader<OnEnemyHit>,
    asset_server: Res<AssetServer>,
) {
    for event in eneny_hit_event.read() {
        if let Ok(enemy_transform) = enemies.get_mut(event.enemy_entity) {
            let parent = commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            left: Val::Px(-990.),
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                        z_index: ZIndex::Global(-100),
                        ..default()
                    },
                    WorldTextUI {
                        lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                        velocity: Vec2::new(0.0, 10.0),
                        position: enemy_transform.translation.truncate(),
                    },
                    Name::new("Enemy UI"),
                ))
                .id();

            let text = commands
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            &format!("{:?}", event.damage as i32),
                            TextStyle {
                                font: asset_server.load(DAMAGE_FONT),
                                font_size: DAMAGE_FONT_SIZE,
                                color: DAMAGE_FONT_COLOR,
                            },
                        )],
                        ..default()
                    },
                    z_index: ZIndex::Local(1),
                    ..default()
                })
                .id();

            let text_shadow = commands
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            &format!("{:?}", event.damage as i32),
                            TextStyle {
                                font: asset_server.load(DAMAGE_FONT),
                                font_size: DAMAGE_FONT_SIZE,
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
                })
                .id();

            commands.entity(parent).push_children(&[text, text_shadow]);
        }
    }
}

fn update_enemy_received_damage_ui(
    mut commands: Commands,
    mut text: Query<(Entity, &mut Style, &mut WorldTextUI)>,
    main_camera: Query<(&Camera, &GlobalTransform)>,
    // render_camera: Query<&Camera>,
    time: Res<Time>,
) {
    let (camera, transform) = main_camera.single();
    // let final_camera = render_camera.single();

    for (entity, mut style, mut world_ui) in &mut text {
        world_ui.lifetime.tick(time.delta());
        if world_ui.lifetime.just_finished() {
            commands.entity(entity).despawn_recursive();
        }

        world_ui.position = world_ui.position + world_ui.velocity * time.delta_seconds();

        if let Some(coords) = camera.world_to_viewport(transform, world_ui.position.extend(0.0)) {
            // let mut coords = coords / Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)
            //     * camera.logical_viewport_size().unwrap();
            // coords.y = camera.logical_viewport_size().unwrap().y - coords.y;
            style.left = Val::Px(coords.x);
            style.top = Val::Px(coords.y);
        }
    }
}
