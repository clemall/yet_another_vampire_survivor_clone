use crate::components::*;
use crate::constants::PROJECTILE_Z_INDEX;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const CLAWS_OFFSET: f32 = 36.0;

#[derive(Component)]
pub struct ClawUpgrade1Spawner;

#[derive(Component)]
pub struct Claw;
pub struct WeaponClawUpgrade1Plugin;

impl Plugin for WeaponClawUpgrade1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_weapon.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_weapon_not_present),
            ),
        );
        app.add_systems(Update, (spawn_attack).run_if(in_state(GameState::Gameplay)));
    }
}

fn run_if_weapon_not_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<ClawUpgrade1Spawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::ClawUpgrade1) && weapon.is_empty()
}

fn spawn_weapon(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        ClawUpgrade1Spawner,
        DelayBetweenAttacks {
            timer: Timer::from_seconds(0.6, TimerMode::Repeating),
        },
        AttackAmmo {
            size: 10 + player_stats.attack_amount,
            default_size: 10,
            amount: 10,
            reload_time: 2.0 * player_stats.attack_reload,
            default_reload_time: 2.0,
        },
        CanAttack,
        Name::new("Claw Spawner"),
    ));
}

fn spawn_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawner: Query<
        (Entity, &mut AttackAmmo),
        (
            With<ClawUpgrade1Spawner>,
            With<CanAttack>,
            Without<AttackSpawnerIsReloading>,
        ),
    >,
    mut player: Query<&Transform, With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_stats: Res<PlayerInGameStats>,
) {
    let player_transform = player.single_mut();

    if let Ok((spawner_entity, mut attack_ammo)) = spawner.get_single_mut() {
        let texture = asset_server.load("claw.png");
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(48.0, 48.0),
            4,
            1,
            Option::from(Vec2::new(1.0, 1.0)),
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        commands.entity(spawner_entity).remove::<CanAttack>();

        let mut counter = 4;

        while attack_ammo.amount > 0 {
            // this weapon should shoot 4 "bullet" at a time
            if counter == 0 {
                return;
            }
            counter -= 1;

            let mut pos_x = player_transform.translation.x;
            let mut pos_y = player_transform.translation.y;

            let mut is_flip = false;

            match attack_ammo.amount % 4 {
                0 => {
                    is_flip = true;
                    pos_x -= CLAWS_OFFSET;
                }
                1 => {
                    is_flip = true;
                    pos_y -= CLAWS_OFFSET;
                }
                2 => {
                    pos_x += CLAWS_OFFSET;
                }
                _ => {
                    pos_y += CLAWS_OFFSET;
                }
            }

            attack_ammo.amount -= 1;

            commands
                .spawn((
                    SpriteBundle {
                        texture: texture.clone(),
                        transform: Transform {
                            translation: Vec3::new(pos_x, pos_y, PROJECTILE_Z_INDEX),
                            scale: Vec3::splat(player_stats.area),
                            ..default()
                        },
                        sprite: Sprite {
                            flip_x: is_flip,
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
                        last: 3,
                        is_repeating: false,
                    },
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    Sensor,
                ))
                .insert((
                    Collider::cuboid(48.0 / 2.0, 48.0 / 2.0),
                    ProjectileBundleCollider::default(),
                    ProjectileLifetime {
                        timer: Timer::from_seconds(
                            0.3 * player_stats.attack_duration,
                            TimerMode::Once,
                        ),
                    },
                    AlreadyHitEnemies { seen: Vec::new() },
                    ProjectileDamage(200.0),
                    ProjectileImpulse(2000.0),
                    Claw,
                    ProjectileFromWeapon(WeaponsTypes::Claw),
                    Projectile,
                    Name::new("Claw Attack"),
                ));
        }
    }
}
