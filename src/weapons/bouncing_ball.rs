use bevy_rapier2d::prelude::*;
use crate::components::*;
use bevy::{
    prelude::*,
};
use crate::math_utils::{find_closest};

pub struct BouncingBallPlugin;

impl Plugin for BouncingBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_bouncing_ball_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_bouncing_ball_present)
            )
        );
         app.add_systems(Update, (
             spawn_bouncing_ball_attack,
             duplicate_ball_on_hit,
             ).run_if(in_state(GameState::Gameplay))
         );

    }
}

fn run_if_bouncing_ball_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<BouncingBallSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::BouncingBall) && weapon.is_empty()
}

fn setup_bouncing_ball_spawner(mut commands: Commands){
    commands.spawn((
        BouncingBallSpawner,
        DelayBetweenAttacks {
            timer:Timer::from_seconds(0.3, TimerMode::Repeating)
        },
        AttackAmmo{
            size: 1,
            amount: 1,
            reload_time: 2.0,
        },
        Name::new("Bouncing ball Spawner"),
    ));
}


fn spawn_bouncing_ball_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<(
        &mut DelayBetweenAttacks,
        &mut AttackAmmo,
    ),(With<BouncingBallSpawner>, Without<AttackReloadDuration>)>,
    mut enemies: Query<(Entity, &Transform),With<Enemy>>,
    time: Res<Time>,
){
    let player_transform = player.single_mut();

    if let Ok(
        (mut attack_timer, mut attack_ammo)
    ) = spawner.get_single_mut(){
        attack_timer.timer.tick(time.delta());

        if attack_timer.timer.just_finished() {
            if attack_ammo.amount == 0 {
                return
            }

            let mut enemies_lens = enemies.transmute_lens::<(Entity, &Transform)>();
            let closed_enemy:Option<Entity> = find_closest(
                player_transform.translation,
                enemies_lens.query()
            );

            if let Some(closed_enemy) = closed_enemy {
                if let Ok((_enemy, enemy_transform)) = enemies.get(closed_enemy) {

                    attack_ammo.amount -= 1;

                    let texture = asset_server.load("bouncing_ball.png");
                    let direction = (player_transform.translation.xy() - enemy_transform.translation.xy()).normalize();
                    commands.spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform{
                                translation: player_transform.translation,
                                ..default()
                            },
                            ..default()
                        },
                        Sensor,
                        Collider::ball(16.0/2.0),
                        ProjectileBundleCollider::default(),
                        ProjectileLifetime {
                            timer:Timer::from_seconds(8.0, TimerMode::Once),
                        },
                        ProjectileDamage(1.0),
                        ProjectileDeleteOnHit,
                        BouncingBall,
                     )).insert((
                         Projectile,
                         ProjectileSpeed(100.0),
                         ProjectileDirection(direction),
                         ProjectileImpulse(700.0),
                         ProjectileType(WeaponsTypes::BouncingBall),
                         Name::new("Bouncing ball Attack"),
                    ));
                }
            }
        }
    }
}

fn duplicate_ball_on_hit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut eneny_hit_event: EventReader<EnemyReceivedDamage>,
) {
    for event in eneny_hit_event.read() {
        if event.weapon_projectile_type != WeaponsTypes::BouncingBall{
            continue
        }
         if let Ok((_enemy_entity, enemy_transform)) = enemies.get(event.enemy_entity){
             for _index in 0..2 {
                 let texture = asset_server.load("bouncing_ball.png");
                 let direction = (enemy_transform.translation.xy() - event.projectile_position.xy()).normalize();
                 commands.spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform{
                                translation: event.projectile_position,
                                ..default()
                            },
                            ..default()
                        },
                        Sensor,
                        Collider::ball(16.0/2.0),
                        ProjectileBundleCollider::default(),
                        ProjectileLifetime {
                            timer:Timer::from_seconds(8.0, TimerMode::Once),
                        },
                        ProjectileDamage(1.0),
                        ProjectileDeleteOnHit,
                        BouncingBall,
                     )).insert((
                     Projectile,
                     ProjectileSpeed(100.0),
                     ProjectileDirection(direction),
                     ProjectileImpulse(700.0),
                     ProjectileType(WeaponsTypes::BouncingBallSplit),
                     Name::new("Bouncing ball duplicate Attack"),
                    ));
             }
         }

    }
}
