use crate::components::*;
use crate::enemies::enemies_bundle::EnemyBundle;
use crate::math_utils::get_random_position_outside_screen;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BossWolfPlugin;

impl Plugin for BossWolfPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_boss_wolf_debug, spawn_boss_wolf));
    }
}

fn spawn_boss_wolf_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut spawn_enemy: EventWriter<SpawnEnemy>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        spawn_enemy.send(SpawnEnemy {
            enemy_types: EnemyTypes::BossWolf,
        });
    }
}

fn spawn_boss_wolf(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawn_enemy: EventReader<SpawnEnemy>,
    camera: Query<&Transform, With<Camera>>,
) {
    let camera = camera.single();
    for event in spawn_enemy.read() {
        if event.enemy_types != EnemyTypes::BossWolf {
            continue;
        }
        let texture = asset_server.load("Canine_White_Run.png");
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(48.0, 32.0),
            4,
            2,
            Option::from(Vec2::new(0.0, 0.0)),
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        commands.spawn((
            EnemyBundle {
                sprite_bundle: SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform {
                        translation: get_random_position_outside_screen(camera.translation.xy())
                            .extend(0.0),
                        rotation: Default::default(),
                        scale: Vec3::new(1.0, 1.0, 0.0),
                    },
                    ..default()
                },
                texture_atlas: TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                },
                animation_indices: AnimationIndices {
                    first: 0,
                    last: 5,
                    is_repeating: true,
                },
                animation_timer: AnimationTimer(Timer::from_seconds(0.14, TimerMode::Repeating)),
                enemy_speed: EnemySpeed(45.0),
                collider: Collider::capsule_x(4.0, 24.0 / 2.0),
                ..default()
            },
            BossWolf,
            // EnemyExperienceDrop(1)
            EnemyBossDrop,
        ));

        // }).with_children(|children| {
        //    children.spawn((
        //        Collider::ball(16.0/2.0),
        //        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        //    ));
        // });
    }
}
