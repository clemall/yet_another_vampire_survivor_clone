use bevy_rapier2d::prelude::*;
use crate::components::*;
use bevy::{
    prelude::*,
};

pub struct SlowDomePlugin;

impl Plugin for SlowDomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_slow_dome_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_slow_dome_present)
            )
        );
         app.add_systems(Update, (
             spawn_slow_dome_attack,
             ).run_if(in_state(GameState::Gameplay))
         );

    }
}

fn run_if_slow_dome_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<SlowDomeSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::SlowDome) && weapon.is_empty()
}

fn setup_slow_dome_spawner(mut commands: Commands){

    commands.spawn((
        SlowDomeSpawner,
        DelayBetweenAttacks {
            timer:Timer::from_seconds(2.0, TimerMode::Repeating),
        },
        AttackAmmo{
            size: 1,
            amount: 1,
            reload_time: 10.0,
        },
        Name::new("Slow Dome Spawner"),
    ));
}


fn spawn_slow_dome_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<(
        &mut DelayBetweenAttacks,
        &mut AttackAmmo,
    ), (With<SlowDomeSpawner>, Without<AttackReloadDuration>)>,
    enemies: Query<(Entity, &Transform),With<Enemy>>,
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
            attack_ammo.amount -= 1;
            
            let mut closed_enemy:Option<Entity>= None;
            let mut closed_enemy_distance:f32 = 999999.0;
            for (entity, enemy_transform) in &enemies {
                let distance = player_transform.translation.distance(enemy_transform.translation);
                if distance < closed_enemy_distance {
                    closed_enemy_distance = distance;
                    closed_enemy = Some(entity);
                }
            }
            
            if let Some(closed_enemy) = closed_enemy{
                if let Ok((_enemy, enemy_transform)) = enemies.get(closed_enemy) {
                    let texture = asset_server.load("shuriken_temp.png");
                    commands.spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform{
                                translation: enemy_transform.translation,
                                scale: Vec3::new(4.0, 4.0, 4.0),
                                ..default()
                            },
                            ..default()
                        },
                        Sensor,
                        Collider::ball(32.0/2.0),
                        ProjectileLifetime {
                            timer:Timer::from_seconds(8.0, TimerMode::Once),
                        },
                        ProjectileDamage(10.0),
                        ProjectileTimeBetweenDamage {
                            timer:Timer::from_seconds(0.33, TimerMode::Repeating),
                        },
                        SlowDome,
                        Projectile,
                        Name::new("Slow dome Attack"),
                    ));
                }
            }
            
        }
    }
}
