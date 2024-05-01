use crate::components::*;
use crate::constants::PROJECTILE_Z_INDEX;
use crate::math_utils::find_closest;
use crate::weapons::shuriken::ShurikenSpawner;
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
            spawn_weapon.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_weapon_not_present),
            ),
        );
        app.add_systems(
            Update,
            (
                spawn_attack,
                apply_stun_on_hit.run_if(run_if_upgrade_stun_on_hit),
            )
                .run_if(in_state(GameState::Gameplay)),
        );

        app.add_systems(
            Update,
            handle_chain_lightning_extra_ammo_update.run_if(
                in_state(GameState::Gameplay)
                    .and_then(run_if_upgrade_extra_ammo)
                    .and_then(run_once()),
            ),
        );
    }
}

fn run_if_weapon_not_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<ChainLightningSpawner>>,
) -> bool {
    player_weapons
        .weapons
        .contains(&WeaponsTypes::ChainLightning)
        && weapon.is_empty()
}

fn run_if_upgrade_stun_on_hit(weapon_upgrades: Res<PlayerUpgradeWeapons>) -> bool {
    weapon_upgrades
        .upgrades
        .contains(&WeaponsUpgradesTypes::ChainLightningStun)
}

fn run_if_upgrade_extra_ammo(weapon_upgrades: Res<PlayerUpgradeWeapons>) -> bool {
    weapon_upgrades
        .upgrades
        .contains(&WeaponsUpgradesTypes::ChainLightningExtraAmmo)
}

fn spawn_weapon(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        ChainLightningSpawner,
        ChainLightning,
        AttackAmmo {
            size: 5 + player_stats.attack_amount,
            amount: 5,
            default_size: 5,
            reload_time: 5.0 * player_stats.attack_reload,
            default_reload_time: 5.0,
        },
        Name::new("Chain Lightning Spawner"),
    ));
}

fn handle_chain_lightning_extra_ammo_update(
    mut spawner: Query<&mut AttackAmmo, With<ChainLightningSpawner>>,
) {
    if let Ok(mut attack_ammo) = spawner.get_single_mut() {
        attack_ammo.default_size += 5;
        attack_ammo.size += 5;
    }
}

fn spawn_attack(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<
        &mut AttackAmmo,
        (
            With<ChainLightningSpawner>,
            Without<AttackSpawnerIsReloading>,
        ),
    >,
    mut enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut eneny_hit_event: EventWriter<OnEnemyHit>,
    weapon_upgrades: Res<PlayerUpgradeWeapons>,
    player_stats: Res<PlayerInGameStats>,
) {
    let player_transform = player.single_mut();

    if let Ok(mut attack_ammo) = spawner.get_single_mut() {
        // Protection from going below 0.
        // AttackReloadDuration can take 1 frame too much before being added to
        // the current spawner
        // if attack_ammo.amount == 0 {
        //     return;
        // }

        // we do not want to target twice the same target, even when having multiple chain lightning
        let mut seen_enemies: Vec<Entity> = Vec::new();

        let amount_of_new_chain = if weapon_upgrades
            .upgrades
            .contains(&WeaponsUpgradesTypes::ChainLightningTriple)
        {
            3
        } else {
            1
        };

        for _ in 0..amount_of_new_chain {
            let texture = asset_server.load("lightning_strike.png");
            let layout = TextureAtlasLayout::from_grid(
                Vec2::new(16.0, 32.0),
                5,
                1,
                Option::from(Vec2::new(0.0, 0.0)),
                None,
            );
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            let mut chain_lightning_ammo = attack_ammo.amount.clone();

            // Start at player
            let mut position_lightning: Vec3 = player_transform.translation;

            while chain_lightning_ammo > 0 {
                chain_lightning_ammo -= 1;
                let mut enemies_lens = enemies.transmute_lens::<(Entity, &Transform)>();
                // get closed enemy
                let closed_enemy: Option<Entity> = find_closest(
                    position_lightning,
                    enemies_lens.query(),
                    300.0,
                    Some(&seen_enemies),
                );

                if let Some(closed_enemy) = closed_enemy {
                    if let Ok((enemy, enemy_transform)) = enemies.get(closed_enemy) {
                        // add current enemy to the list
                        seen_enemies.push(enemy);

                        // draw
                        let lightning_direction = (enemy_transform.translation.xy()
                            - position_lightning.xy())
                        .normalize();
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
                                        PROJECTILE_Z_INDEX,
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

                        eneny_hit_event.send(OnEnemyHit {
                            damage: 50.0 * player_stats.power,
                            enemy_entity: enemy,
                            projectile_position: enemy_transform.translation,
                            impulse: None,
                            projectile_type: ProjectileTypes::ChainLightning,
                        });
                    }
                }
            }
        }
        // set ammo to 0 after shooting all chain lightning;
        attack_ammo.amount = 0;
    }
}

fn apply_stun_on_hit(
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
    mut eneny_hit_event: EventReader<OnEnemyHit>,
) {
    for event in eneny_hit_event.read() {
        if event.projectile_type != ProjectileTypes::ChainLightning {
            continue;
        }
        if let Ok(enemy_entity) = enemies.get(event.enemy_entity) {
            commands.entity(enemy_entity).try_insert(StunAura {
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            });
        }
    }
}
