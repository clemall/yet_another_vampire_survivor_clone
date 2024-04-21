use crate::components::*;
use crate::math_utils::find_closest;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::f32::consts::PI;

#[derive(Component)]
pub struct ChainLightningSpawner;
#[derive(Component)]
pub struct ChainLightning;

pub struct ChainLightningPlugin;

impl Plugin for ChainLightningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_chain_lightning_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>
                    .and_then(run_if_chain_lightning_present),
            ),
        );
        app.add_systems(
            Update,
            (spawn_chain_lightning_attack, chain_lightning_update_stats)
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_chain_lightning_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<ChainLightningSpawner>>,
) -> bool {
    player_weapons
        .weapons
        .contains(&WeaponsTypes::ChainLightning)
        && weapon.is_empty()
}

fn setup_chain_lightning_spawner(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        ChainLightningSpawner,
        ChainLightning,
        AttackAmmo {
            size: 100,
            amount: 100,
            reload_time: 5.0 * player_stats.attack_reload,
        },
        Name::new("Chain Lightning Spawner"),
    ));
}

fn chain_lightning_update_stats(
    mut attack_ammos: Query<&mut AttackAmmo, With<ChainLightningSpawner>>,
    player_stats: Res<PlayerInGameStats>,
) {
    if !player_stats.is_changed() {
        return;
    }
    for mut attack_ammo in &mut attack_ammos {
        attack_ammo.reload_time = 5.0 * player_stats.attack_reload;
    }
}

fn spawn_chain_lightning_attack(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<
        &mut AttackAmmo,
        (With<ChainLightningSpawner>, Without<AttackReloadDuration>),
    >,
    mut enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut enemy_received_damage: EventWriter<EnemyReceivedDamage>,
) {
    let player_transform = player.single_mut();

    if let Ok(mut attack_ammo) = spawner.get_single_mut() {
        // Protection from going below 0.
        // AttackReloadDuration can take 1 frame too much before being added to
        // the current spawner
        if attack_ammo.amount == 0 {
            return;
        }

        let texture = asset_server.load("lightning_strike.png");
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(16.0, 32.0),
            5,
            1,
            Option::from(Vec2::new(0.0, 0.0)),
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let mut seen_enemies: Vec<Entity> = Vec::new();
        // Start at player
        let mut position_lightning: Vec3 = player_transform.translation;

        while attack_ammo.amount > 0 {
            attack_ammo.amount -= 1;
            let mut enemies_lens = enemies.transmute_lens::<(Entity, &Transform)>();
            // get closed enemy
            let closed_enemy: Option<Entity> = find_closest(
                position_lightning,
                enemies_lens.query(),
                Some(&seen_enemies),
            );

            if let Some(closed_enemy) = closed_enemy {
                if let Ok((enemy, enemy_transform)) = enemies.get(closed_enemy) {
                    // add current enemy to the list
                    seen_enemies.push(enemy);

                    // draw
                    let lightning_direction =
                        (enemy_transform.translation.xy() - position_lightning.xy()).normalize();
                    let lightning_distance = enemy_transform
                        .translation
                        .xy()
                        .distance(position_lightning.xy());
                    let scale_y = lightning_distance / 32.0;

                    commands.spawn((
                        SpriteBundle {
                            texture: texture.clone(),
                            transform: Transform {
                                translation: Vec3::new(
                                    position_lightning.x,
                                    position_lightning.y,
                                    1.0,
                                ),
                                rotation: Quat::from_rotation_z(
                                    lightning_direction.to_angle() - PI / 2.0,
                                ),
                                scale: Vec3::new(1.0, scale_y, 1.0),
                                ..default()
                            },
                            sprite: Sprite {
                                anchor: Anchor::BottomCenter,
                                ..default()
                            },
                            ..default()
                        },
                        // ImageScaleMode::Tiled {
                        //     tile_y: true,
                        //     tile_x: false,
                        //     stretch_value: 1.0,
                        // },
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: 0,
                        },
                        AnimationIndices {
                            first: 0,
                            last: 4,
                            is_repeating: true,
                        },
                        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                        Projectile,
                        ProjectileFixedScale,
                        ProjectileLifetime {
                            timer: Timer::from_seconds(0.5, TimerMode::Once),
                        },
                    ));

                    // move position to the one from the enemy
                    position_lightning = enemy_transform.translation.clone();

                    enemy_received_damage.send(EnemyReceivedDamage {
                        damage: 50.0,
                        enemy_entity: enemy,
                        projectile_position: enemy_transform.translation,
                        impulse: None,
                        weapon_projectile_type: WeaponsTypes::ChainLightning,
                    });
                }
            }
        }
    }
}
