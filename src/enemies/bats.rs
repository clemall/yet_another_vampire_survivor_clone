use crate::components::*;
use crate::enemies::enemies_bundle::EnemyBundle;
use crate::math_utils::get_random_position_outside_screen;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BatPlugin;

impl Plugin for BatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_bats_debug, spawn_bats));
    }
}

fn spawn_bats_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut spawn_enemy: EventWriter<SpawnEnemy>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyO) || keyboard_input.pressed(KeyCode::KeyP) {
        spawn_enemy.send(SpawnEnemy {
            enemy_types: EnemyTypes::Bat,
        });
    }
}

fn spawn_bats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawn_enemy: EventReader<SpawnEnemy>,
    player: Query<&Transform, With<Player>>,
) {
    let camera = player.single();
    for event in spawn_enemy.read() {
        if event.enemy_types != EnemyTypes::Bat {
            continue;
        }
        let texture = asset_server.load("Bat_Fly.png");
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(32.0, 32.0),
            4,
            1,
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
                    last: 3,
                    is_repeating: true,
                },
                enemy_speed: EnemySpeed(30.0),
                collider: Collider::ball(14.0 / 2.0),
                ..default()
            },
            Bat,
            EnemyExperienceDrop(1),
        ));
        // }).with_children(|children| {
        //    children.spawn((
        //        Collider::ball(16.0/2.0),
        //        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        //    ));
        // });
    }
}
