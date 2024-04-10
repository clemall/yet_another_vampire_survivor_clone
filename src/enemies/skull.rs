use crate::components::*;
use crate::enemies::enemies_bundle::EnemyBundle;
use crate::math_utils::get_random_position_outside_screen;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct SkullPlugin;

impl Plugin for SkullPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_skull_debug, spawn_skulls));
    }
}

fn spawn_skull_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut spawn_enemy: EventWriter<SpawnEnemy>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyK) || keyboard_input.pressed(KeyCode::KeyL) {
        spawn_enemy.send(SpawnEnemy {
            enemy_types: EnemyTypes::Skull,
        });
    }
}

fn spawn_skulls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawn_enemy: EventReader<SpawnEnemy>,
    camera: Query<&Transform, With<Camera>>,
) {
    let camera = camera.single();
    for event in spawn_enemy.read() {
        if event.enemy_types != EnemyTypes::Skull {
            continue;
        }
        let texture = asset_server.load("Bones_SingleSkull_Fly.png");
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(32.0, 32.0),
            8,
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
                    last: 7,
                    is_repeating: true,
                },
                collider: Collider::capsule_x(3.0, 8.0 / 2.0),
                ..default()
            },
            Skull,
        ));
        // }).with_children(|children| {
        //    children.spawn((
        //        Collider::capsule_x(3.0,8.0/2.0),
        //        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        //    ));
        // });
    }
}
