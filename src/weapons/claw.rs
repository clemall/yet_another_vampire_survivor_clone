use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const CLAWS_POSITION_X: f32 = 28.0;

#[derive(Component)]
pub struct ClawSpawner;

#[derive(Component)]
pub struct Claw;
pub struct WeaponClawPlugin;

impl Plugin for WeaponClawPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_claw_spawner
                .run_if(resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_claw_present)),
        );
        app.add_systems(
            Update,
            (spawn_claw_attack,).run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_claw_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<ClawSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::Claw) && weapon.is_empty()
}

fn setup_claw_spawner(mut commands: Commands, player_stats: Res<PlayerInGameStats>) {
    commands.spawn((
        ClawSpawner,
        DelayBetweenAttacks {
            timer: Timer::from_seconds(0.6, TimerMode::Repeating),
        },
        AttackAmmo {
            size: 2 + player_stats.attack_amount,
            default_size: 2,
            amount: 2,
            reload_time: 2.0 * player_stats.attack_reload,
            default_reload_time: 2.0,
        },
        CanAttack,
        ProjectileBendLeftOrRight(true),
        Name::new("Claw Spawner"),
    ));
}

fn spawn_claw_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawner: Query<
        (Entity, &mut AttackAmmo, &mut ProjectileBendLeftOrRight),
        (
            With<ClawSpawner>,
            With<CanAttack>,
            Without<AttackSpawnerIsReloading>,
        ),
    >,
    mut player: Query<&Transform, With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_stats: Res<PlayerInGameStats>,
) {
    let player_transform = player.single_mut();

    if let Ok((spawner_entity, mut attack_ammo, mut projectile_orientation)) = spawner.get_single_mut() {
        let texture = asset_server.load("claw.png");
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(48.0, 48.0),
            2,
            1,
            Option::from(Vec2::new(1.0, 1.0)),
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let mut pos_x = player_transform.translation.x;

        let is_flip = match projectile_orientation.0 {
            true => {
                pos_x -= CLAWS_POSITION_X;
                true
            }
            false => {
                pos_x += CLAWS_POSITION_X;
                false
            }
        };

        **projectile_orientation = !projectile_orientation.0;

        attack_ammo.amount -= 1;
        commands.entity(spawner_entity).remove::<CanAttack>();

        commands
            .spawn((
                SpriteBundle {
                    texture,
                    // transform: Transform::from_xyz(pos_x, player_transform.translation.y, 1.0),
                    transform: Transform {
                        translation: Vec3::new(pos_x, player_transform.translation.y, 1.0),
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
                    layout: texture_atlas_layout,
                    index: 0,
                },
                AnimationIndices {
                    first: 0,
                    last: 1,
                    is_repeating: false,
                },
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                Sensor,
            ))
            .insert((
                Collider::cuboid(48.0 / 2.0, 48.0 / 2.0),
                ProjectileBundleCollider::default(),
                ProjectileLifetime {
                    timer: Timer::from_seconds(0.3 * player_stats.attack_duration, TimerMode::Once),
                },
                AlreadyHitEnemies { seen: Vec::new() },
                ProjectileDamage(5.0),
                ProjectileImpulse(2000.0),
                Claw,
                ProjectileFromWeapon(WeaponsTypes::Claw),
                Projectile,
                Name::new("Claw Attack"),
            ));
    }
}

// fn claw_attack_animation_and_collider(
//     mut claws: Query<(&mut Transform, &ProjectileLifetime), With<Claw>>,
// ) {
//     for (mut tranform, attack_duration) in &mut claws {
//         // transform claw attack
//         tranform.scale.x = (attack_duration.timer.fraction() * 0.2) + 0.8;
//         tranform.scale.y = (attack_duration.timer.fraction() * 0.2) + 0.8;
//     }
// }
