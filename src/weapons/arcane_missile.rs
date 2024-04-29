use crate::components::*;
use crate::constants::PROJECTILE_Z_INDEX;
use crate::math_utils::{find_circle_circle_intersections, find_closest};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::SliceRandom;
use rand::Rng;

#[derive(Component)]
pub struct ArcaneMissileSpawner;

#[derive(Component)]
pub struct ArcaneMissile;
pub struct ArcaneMissilePlugin;

impl Plugin for ArcaneMissilePlugin {
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
                handle_arcane_missile_split_on_hit.run_if(run_if_upgrade_split_is_present),
                handle_arcane_missile_explosion_hit.run_if(run_if_upgrade_explosion_is_present),
            )
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_weapon_not_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<ArcaneMissileSpawner>>,
) -> bool {
    player_weapons
        .weapons
        .contains(&WeaponsTypes::ArcaneMissile)
        && weapon.is_empty()
}

fn run_if_upgrade_split_is_present(weapon_upgrades: Res<PlayerUpgradeWeapons>) -> bool {
    weapon_upgrades
        .upgrades
        .contains(&WeaponsUpgradesTypes::ArcaneMissileSplit)
}

fn run_if_upgrade_explosion_is_present(weapon_upgrades: Res<PlayerUpgradeWeapons>) -> bool {
    weapon_upgrades
        .upgrades
        .contains(&WeaponsUpgradesTypes::ArcaneMissileExplosion)
}

fn spawn_weapon(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        ArcaneMissileSpawner,
        DelayBetweenAttacks {
            timer: Timer::from_seconds(0.4, TimerMode::Repeating),
        },
        AttackAmmo {
            size: 3 + player_stats.attack_amount,
            amount: 3,
            default_size: 3,
            reload_time: 2.0 * player_stats.attack_reload,
            default_reload_time: 2.0,
        },
        CanAttack,
        ProjectileBendLeftOrRight(true),
        Name::new("Arcane missile Spawner"),
    ));
}

fn spawn_attack(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<
        (Entity, &mut AttackAmmo, &mut ProjectileBendLeftOrRight),
        (
            With<ArcaneMissileSpawner>,
            With<CanAttack>,
            Without<AttackSpawnerIsReloading>,
        ),
    >,
    mut enemies: Query<(Entity, &Transform), With<Enemy>>,
    player_stats: Res<PlayerInGameStats>,
    weapon_upgrades: Res<PlayerUpgradeWeapons>,
) {
    let player_transform = player.single_mut();

    if let Ok((spawner_entity, mut attack_ammo, mut projectile_orientation)) =
        spawner.get_single_mut()
    {
        // get closed enemy
        let mut enemies_lens = enemies.transmute_lens::<(Entity, &Transform)>();
        let closed_enemy: Option<Entity> = find_closest(
            player_transform.translation,
            enemies_lens.query(),
            300.0,
            None,
        );

        if let Some(closed_enemy) = closed_enemy {
            let texture = asset_server.load("arcane-missile.png");
            let layout = TextureAtlasLayout::from_grid(Vec2::new(100.0, 100.0), 30, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            if let Ok((entity, enemy_transform)) = enemies.get(closed_enemy) {
                let distance_enemy_player = enemy_transform
                    .translation
                    .distance(player_transform.translation);

                let (control_point_1, control_point_2) = find_circle_circle_intersections(
                    player_transform.translation,
                    distance_enemy_player / 2.0 + 15.0,
                    enemy_transform.translation,
                    distance_enemy_player / 2.0 + 15.0,
                );

                let control_point = if projectile_orientation.0 {
                    control_point_1
                } else {
                    control_point_2
                };

                **projectile_orientation = !projectile_orientation.0;

                attack_ammo.amount -= 1;
                commands.entity(spawner_entity).remove::<CanAttack>();

                let projectile_id = commands
                    .spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform {
                                translation: Vec3::new(
                                    player_transform.translation.x,
                                    player_transform.translation.y,
                                    PROJECTILE_Z_INDEX,
                                ),
                                scale: Vec3::splat(player_stats.area),
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            layout: texture_atlas_layout,
                            index: 0,
                        },
                        AnimationIndices {
                            first: 0,
                            last: 1,
                            is_repeating: true,
                        },
                        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                        Sensor,
                        Collider::ball(18.0 / 2.0),
                        ArcaneMissile,
                    ))
                    .insert((
                        Projectile,
                        ProjectileFromWeapon(WeaponsTypes::ArcaneMissile),
                        ProjectileDamage(50.0),
                        ProjectileTarget(entity),
                        ProjectileOrigin(player_transform.translation),
                        ProjectileControlPoint(control_point),
                        ProjectileImpulse(120.0),
                        AlreadyHitEnemies { seen: Vec::new() },
                        ProjectileSpeedAsDuration {
                            timer: Timer::from_seconds(0.3, TimerMode::Once),
                        },
                        ProjectileLifetime {
                            timer: Timer::from_seconds(0.3001, TimerMode::Once),
                        },
                        ProjectileBundleCollider::default(),
                        Name::new("Arcane missile Attack"),
                    ))
                    .id();

                if weapon_upgrades
                    .upgrades
                    .contains(&WeaponsUpgradesTypes::ArcaneMissilePierce)
                {
                    commands.entity(projectile_id).insert(ProjectilePierce);
                }
            }
        }
    }
}

fn handle_arcane_missile_split_on_hit(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut eneny_hit_event: EventReader<OnEnemyHit>,
    player_stats: Res<PlayerInGameStats>,
    weapon_upgrades: Res<PlayerUpgradeWeapons>,
) {
    for event in eneny_hit_event.read() {
        if event.weapon_projectile_type != WeaponsTypes::ArcaneMissile {
            continue;
        }

        for _index in 0..2 + player_stats.attack_amount {
            let enemies_entity: Vec<Entity> =
                enemies.transmute_lens::<Entity>().query().iter().collect();
            // try to find another enemy or default to the same one
            // also set the already hit vec
            let picked_enemy: Option<&Entity> = enemies_entity.choose(&mut rand::thread_rng());
            let mut seen = Vec::new();
            let picked_enemy = match picked_enemy {
                None => event.enemy_entity,
                Some(picked_enemy) => {
                    seen.push(event.enemy_entity);
                    *picked_enemy
                }
            };

            let texture = asset_server.load("arcane-missile.png");
            let layout = TextureAtlasLayout::from_grid(Vec2::new(100.0, 100.0), 30, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            if let Ok((entity, enemy_transform)) = enemies.get(picked_enemy) {
                let distance_enemy_projectiler = enemy_transform
                    .translation
                    .distance(event.projectile_position);

                let (control_point_1, control_point_2) = find_circle_circle_intersections(
                    event.projectile_position,
                    distance_enemy_projectiler / 2.0 + 30.0,
                    enemy_transform.translation,
                    distance_enemy_projectiler / 2.0 + 30.0,
                );

                let control_point = if rand::thread_rng().gen_bool(1.0 / 2.0) {
                    control_point_1
                } else {
                    control_point_2
                };

                let projectile_id = commands
                    .spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform {
                                translation: Vec3::new(
                                    event.projectile_position.x,
                                    event.projectile_position.y,
                                    PROJECTILE_Z_INDEX,
                                ),
                                scale: Vec3::splat(player_stats.area),
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            layout: texture_atlas_layout,
                            index: 0,
                        },
                        AnimationIndices {
                            first: 0,
                            last: 1,
                            is_repeating: true,
                        },
                        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                        Sensor,
                        Collider::ball(18.0 / 2.0),
                        ArcaneMissile,
                    ))
                    .insert((
                        Projectile,
                        ProjectileFromWeapon(WeaponsTypes::ArcaneMissileSplit),
                        ProjectileDamage(25.0),
                        ProjectileTarget(entity),
                        ProjectileOrigin(event.projectile_position),
                        ProjectileControlPoint(control_point),
                        ProjectileImpulse(120.0),
                        AlreadyHitEnemies { seen },
                        ProjectileSpeedAsDuration {
                            timer: Timer::from_seconds(0.3, TimerMode::Once),
                        },
                        ProjectileLifetime {
                            timer: Timer::from_seconds(0.3001, TimerMode::Once),
                        },
                        ProjectileBundleCollider::default(),
                        Name::new("Arcane missile Attack"),
                    ))
                    .id();

                if weapon_upgrades
                    .upgrades
                    .contains(&WeaponsUpgradesTypes::ArcaneMissilePierce)
                {
                    commands.entity(projectile_id).insert(ProjectilePierce);
                }
            }
        }
    }
}

fn handle_arcane_missile_explosion_hit(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut eneny_hit_event: EventReader<OnEnemyHit>,
    player_stats: Res<PlayerInGameStats>,
) {
    for event in eneny_hit_event.read() {
        if event.weapon_projectile_type != WeaponsTypes::ArcaneMissile
            && event.weapon_projectile_type != WeaponsTypes::ArcaneMissileSplit
        {
            continue;
        }

        let texture = asset_server.load("arcane-missile-explosion.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 11, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        commands
            .spawn((
                SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: Vec3::new(
                            event.projectile_position.x,
                            event.projectile_position.y,
                            PROJECTILE_Z_INDEX,
                        ),
                        scale: Vec3::splat(player_stats.area),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
                AnimationIndices {
                    first: 0,
                    last: 10,
                    is_repeating: false,
                },
                AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
                Sensor,
                Collider::ball(55.0 / 2.0),
                ArcaneMissile,
            ))
            .insert((
                Projectile,
                ProjectileFromWeapon(WeaponsTypes::ArcaneMissileExplosion),
                ProjectileDamage(80.0),
                ProjectileOrigin(event.projectile_position),
                ProjectileImpulse(120.0),
                AlreadyHitEnemies { seen: Vec::new() },
                ProjectilePierce,
                ProjectileLifetime {
                    timer: Timer::from_seconds(0.4, TimerMode::Once),
                },
                ProjectileBundleCollider::default(),
                Name::new("Arcane missile explosion"),
            ));
        // }
    }
}
