use crate::components::*;
use crate::constants::PROJECTILE_Z_INDEX;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct FireBootSpawner;

#[derive(Component)]
pub struct FireBoot;

pub struct FireBootsPlugin;

impl Plugin for FireBootsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_weapon.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_weapon_not_present),
            ),
        );
        app.add_systems(
            Update,
            (spawn_attack,).run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_weapon_not_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<FireBootSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::FireBoots) && weapon.is_empty()
}

fn spawn_weapon(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        FireBootSpawner,
        DelayBetweenAttacks {
            timer: Timer::from_seconds(0.3, TimerMode::Repeating),
        },
        AttackAmmo {
            size: 12 + player_stats.attack_amount,
            amount: 12,
            default_size: 12,
            reload_time: 5.0 * player_stats.attack_reload,
            default_reload_time: 5.0,
        },
        CanAttack,
        Name::new("Fire boots Spawner"),
    ));
}

fn spawn_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<
        (Entity, &mut AttackAmmo),
        (
            With<FireBootSpawner>,
            With<CanAttack>,
            Without<AttackSpawnerIsReloading>,
        ),
    >,
    player_stats: Res<PlayerInGameStats>,
) {
    let player_transform = player.single_mut();

    if let Ok((spawner_entity, mut attack_ammo)) = spawner.get_single_mut() {
        let texture = asset_server.load("fire-boots.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 8, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        attack_ammo.amount -= 1;
        commands.entity(spawner_entity).remove::<CanAttack>();

        commands
            .spawn((
                SpriteBundle {
                    texture: texture.clone(),
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
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                },
                AnimationIndices {
                    first: 0,
                    last: 7,
                    is_repeating: true,
                },
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                Sensor,
                Collider::ball(28.0 / 2.0),
                ProjectileBundleCollider::default(),
            ))
            .insert((
                Projectile,
                ProjectileType(ProjectileTypes::FireBoots),
                FireBoot,
                ProjectileDamage(30.0),
                ProjectileImpulse(20.0),
                ProjectilePierce,
                ProjectileLifetime {
                    timer: Timer::from_seconds(5.0 * player_stats.attack_duration, TimerMode::Once),
                },
                ProjectileTimeBetweenDamage {
                    timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                },
                // ProjectileFollowPlayer,
                // ProjectileSpeed(20.0),
                ProjectileOrigin(player_transform.translation),
                Name::new("Fire boots Attack"),
            ));
    }
}
