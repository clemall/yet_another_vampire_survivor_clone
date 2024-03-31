use std::f32::consts::PI;
use crate::components::*;
use bevy::{
    prelude::*,
};
use bevy::sprite::Anchor;

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
            size: 100,
            amount: 100,
            reload_time: 5.0,
        },
        Name::new("Chain Lightning Spawner"),
    ));
}


fn spawn_chain_lightning_attack(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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

        let texture = asset_server.load("lightning_strike.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 32.0), 5, 1, Option::from(Vec2::new(0.0, 0.0)), None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);


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

                    // draw
                    let lightning_direction = (enemy_transform.translation.xy() - position_lightning.xy()).normalize();
                    let lightning_distance = enemy_transform.translation.xy().distance(position_lightning.xy());
                    let scale_y= lightning_distance / 32.0;

                    commands.spawn((
                        SpriteBundle {
                            texture:texture.clone(),
                            transform: Transform{
                                translation: Vec3::new(position_lightning.x, position_lightning.y, 1.0),
                                rotation:Quat::from_rotation_z(lightning_direction.to_angle() - PI/2.0),
                                scale:Vec3::new(1.0, scale_y, 1.0),
                                ..default()
                            },
                            sprite: Sprite{
                                anchor:Anchor::BottomCenter,
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
                        AnimationIndices { first: 0, last: 4, is_repeating: true },
                        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                        Projectile,
                        ProjectileLifetime {
                            timer:Timer::from_seconds(0.5, TimerMode::Once),
                        },
                    ));

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
