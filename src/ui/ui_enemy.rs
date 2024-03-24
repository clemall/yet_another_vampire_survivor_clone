use bevy::prelude::*;
use crate::components::*;

pub struct UiEnemyPlugin;

impl Plugin for UiEnemyPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_claw);
        // app.add_systems(Update,(claw_attack_facing, claw_attack, claw_damage));
        app.add_systems(Update, (update_world_text, update_world_text));
    }
}


pub fn spawn_world_text(commands: &mut Commands,  position: Vec2, text: &str) {
    let position = position + Vec2::new(-0.2, 1.4);

    let parent = (
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
            position,
        },
        Name::new("Enemy UI"),
    );

    let text = TextBundle::from_section(
        text,
        TextStyle {
            font: Default::default(),
            font_size: 32.0,
            color: Color::rgb(0.95, 0.2, 0.2),
        },
    );

    commands.spawn(parent).with_children(|commands| {
        commands.spawn(text);
    });
}

fn update_world_text(
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

        if let Some(mut coords) = camera.world_to_viewport(transform, world_ui.position.extend(0.0)) {
            // let mut coords = coords / Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)
            //     * camera.logical_viewport_size().unwrap();
            // coords.y = camera.logical_viewport_size().unwrap().y - coords.y;
            style.left = Val::Px(coords.x);
            style.top = Val::Px(coords.y);

        }
    }
}
