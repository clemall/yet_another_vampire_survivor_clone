use std::f32::consts::{TAU};
use bevy_rapier2d::prelude::*;
use crate::components::*;
use bevy::{
    prelude::*,
};

pub struct ChainLightningPlugin;

impl Plugin for ChainLightningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_chain_lightning_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_chain_lightning_present)
            )
        );
         app.add_systems(Update, (
             spawn_chain_lightning_attack,
             ).run_if(in_state(GameState::Gameplay))
         );

    }
}

fn run_if_chain_lightning_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<ChainLightningSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::ChainLightning) && weapon.is_empty()
}

fn setup_chain_lightning_spawner(mut commands: Commands){

    commands.spawn((
        ChainLightningSpawner,
        AttackAmmo{
            size: 3,
            amount: 3,
            reload_time: 5.0,
        },
        Name::new("Chain Lightning Spawner"),
    ));
}


fn spawn_chain_lightning_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut spawner: Query<&mut AttackAmmo,(With<ChainLightningSpawner>, Without<AttackReloadDuration>)>,
    enemies: Query<(Entity, &Transform),With<Enemy>>,
    mut enemy_received_damage: EventWriter<EnemyReceivedDamage>,
){
    let player_transform = player.single_mut();

    if let Ok(mut attack_ammo) = spawner.get_single_mut(){
        // Protection from going below 0.
        // AttackReloadDuration can take 1 frame too much before being added to
        // the current spawner
        if attack_ammo.amount == 0 {
            return
        }

        // let texture = asset_server.load("shuriken_temp.png");

        let mut seen_enemies:Vec<Entity> = Vec::new();
        // Start at player
        let mut position_lightning:Vec3 = player_transform.translation;

        while attack_ammo.amount > 0 {
            attack_ammo.amount -= 1;


            let mut closed_enemy:Option<Entity>= None;
            let mut closed_enemy_distance:f32 = 999999.0;
            for (entity, enemy_transform) in &enemies {
                if seen_enemies.contains(&entity){
                    continue;
                }
                let distance = position_lightning.distance(enemy_transform.translation);
                if distance < closed_enemy_distance {
                    closed_enemy_distance = distance;
                    closed_enemy = Some(entity);
                }
            }

            if let Some(closed_enemy) = closed_enemy{
                if let Ok((enemy, enemy_transform)) = enemies.get(closed_enemy) {
                    // add current enemy to the list
                    seen_enemies.push(enemy);

                    // move position to the one from the enemy
                    position_lightning = enemy_transform.translation.clone();

                    enemy_received_damage.send(
                        EnemyReceivedDamage{
                            damage: 50.0,
                            enemy_entity: enemy,
                        }

                    );
                }
            }


            //
            //
            // commands.spawn((
            //     SpriteBundle {
            //         texture: texture.clone(),
            //         transform: Transform{
            //             translation: Vec3::new(player_transform.translation.x, player_transform.translation.y, 1.0),
            //             ..default()
            //         },
            //         ..default()
            //     },
            //     Sensor,
            //     Collider::ball(32.0/2.0),
            //     ArcaneMissile,
            // )).insert((
            //     Projectile,
            //     ProjectileSpeed(2.0),
            //     // ProjectileDirection(direction),
            //     // ProjectileRotateAroundPlayer{
            //     //     angle,
            //     //     distance: 40.0,
            //     // },
            //     // ProjectileSpiralAroundPlayer{
            //     //     angle,
            //     //     distance: 0.0,
            //     //     spiral_speed: 60.0,
            //     // },
            //     ProjectileDamage(10.0),
            //     ProjectileImpulse(200.0),
            //     ProjectileLifetime {
            //         timer:Timer::from_seconds(10.0, TimerMode::Once),
            //     },
            //     AlreadyHitEnemies{seen:Vec::new()},
            //     ProjectileRotateOnSelf,
            //     ProjectileOrigin(player_transform.translation),
            //     Name::new("Shuriken Attack"),
            // ));
        }
    }
}
