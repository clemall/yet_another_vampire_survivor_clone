use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::enemies::enemy::{damage_enemy};
use bevy::{
    prelude::*,
};
use crate::math_utils::{find_circle_circle_intersections, simple_bezier};

pub struct ArcaneMissilePlugin;

impl Plugin for ArcaneMissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_arcane_missile_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_arcane_missile_present)
            )
        );
         app.add_systems(Update, (
             spawn_arcane_missile_attack,
             move_arcane_missile,
             start_reload_arcane_missile,
             reloading_arcane_missile,
             arcane_missile_damage,
             ).run_if(in_state(GameState::Gameplay))
         );

    }
}

fn run_if_arcane_missile_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<ArcaneMissileSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::ArcaneMissile) && weapon.is_empty()
}

fn setup_arcane_missile_spawner(mut commands: Commands){

    commands.spawn((
        ArcaneMissileSpawner,
        AttackTimer{
            timer:Timer::from_seconds(0.1, TimerMode::Repeating)
        },
        AttackAmmo{
            size: 3,
            current: 3,
        },
        Name::new("Arcane missile Spawner"),
    ));
}

fn start_reload_arcane_missile(
    mut commands: Commands,
    mut arcane_missile_spawner: Query<(
        Entity,
        &mut AttackAmmo), (With<ArcaneMissileSpawner>, Without<AttackReload>)>,
){
    if let Ok((entity, attack_ammo)) = arcane_missile_spawner.get_single_mut(){
        if attack_ammo.current == 0 {
            commands.entity(entity).insert(AttackReload{
                timer:Timer::from_seconds(3.0, TimerMode::Once)
            },);
        }

    }
}

fn reloading_arcane_missile(
    mut commands: Commands,
    mut arcane_missile_spawner: Query<(
        Entity,
        &mut AttackReload,
        &mut AttackAmmo), (With<ArcaneMissileSpawner>, With<AttackReload>)>,
    time: Res<Time>,
){
    if let Ok((entity, mut attack_reload, mut attack_ammo)) = arcane_missile_spawner.get_single_mut(){
        attack_reload.timer.tick(time.delta());

        if attack_reload.timer.just_finished() {
            commands.entity(entity).remove::<AttackReload >();
        }

        attack_ammo.current = attack_ammo.size;
    }
}

fn spawn_arcane_missile_attack(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut player: Query<&Transform, With<Player>>,
    mut arcane_missile_spawner: Query<(&mut AttackTimer, &mut AttackAmmo),(With<ArcaneMissileSpawner>, Without<AttackReload>)>,
    enemies: Query<(Entity, &Transform),With<Enemy>>,
    mut projectile_offset_bool: ResMut<ProjectileOffsetGoesLeft>,
    time: Res<Time>,
){
    let player_transform = player.single_mut();

    if let Ok((mut attack_timer,
              mut attack_ammo
              )) = arcane_missile_spawner.get_single_mut(){

        attack_timer.timer.tick(time.delta());

        if attack_timer.timer.just_finished() {

            // get closed enemy
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
                let texture = asset_server.load("arcane_missile.png");
                let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 19.0), 2, 1, Option::from(Vec2::new(1.0, 0.0)), None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                if let Ok((entity, enemy_transform)) = enemies.get(closed_enemy) {
                    let distance_enemy_player = enemy_transform.translation.distance(player_transform.translation);

                    let (
                        control_point_1,
                        control_point_2
                    ) = find_circle_circle_intersections(
                        player_transform.translation,
                        distance_enemy_player/2.0 + 15.0,
                        enemy_transform.translation,
                        distance_enemy_player/2.0 + 15.0,
                    );

                    let control_point= if projectile_offset_bool.0{
                        control_point_1
                    }
                    else {
                        control_point_2
                    };

                    projectile_offset_bool.0 = !projectile_offset_bool.0;

                    commands.spawn((
                            SpriteBundle {
                                texture,
                                transform: Transform{
                                    translation: Vec3::new(player_transform.translation.x, player_transform.translation.y, 1.0),
                                    scale: Vec3::new(0.5, 0.5, 0.5),
                                    ..default()
                                },
                                ..default()
                            },
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 0,
                            },
                            AnimationIndices { first: 0, last: 1, is_repeating: true },
                            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                            Sensor,
                            Collider::ball(32.0/2.0),
                            ArcaneMissile{
                                damage: 50.0,
                            },
                            ProjectileTarget(entity),
                            ProjectileVelocity(Vec2::new(0.0, 0.0)),
                            ProjectileSpeed(10.0),
                            ProjectileOrigin(player_transform.translation),
                            ProjectileControlPoint(control_point),
                            AttackDuration{
                                timer:Timer::from_seconds(0.3, TimerMode::Once),
                            },
                            Name::new("Arcane missile Attack"),
                        )
                    );
                }

                // check reload
                attack_ammo.current -= 1;

            }
        }
    }
}


fn arcane_missile_damage(
    mut commands: Commands,
    mut arcane_missiles: Query<(
        Entity,
        &Collider,
        &GlobalTransform,
        &ArcaneMissile,
    ), Without<ColliderDisabled>>,
    mut enemy: Query<(&mut Health, &Transform), With<Enemy>>,
    mut player: Query<&Transform, With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    for (arcane_missile_entity, collider, transform, arcane_missile) in &mut arcane_missiles {

        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok((health, transform)) = enemy.get_mut(entity) {
                    damage_enemy(&mut commands,entity, health, transform, arcane_missile.damage);

                    let player_transform = player.single_mut();
                    let direction:Vec2 = transform.translation.truncate() -player_transform.translation.truncate();
                    commands.entity(entity).try_insert(ExternalImpulse   {
                        impulse: direction.normalize() * 2000.0,
                        torque_impulse: 0.0,
                    },);

                    // delete projectile
                    commands.entity(arcane_missile_entity).despawn_recursive();


                }
                true
            },
        );

    }
}




fn move_arcane_missile(
    mut commands: Commands,
    mut arcane_missiles: Query<(
        Entity,
        &mut Transform,
        &mut Sprite,
        &mut ProjectileVelocity,
        &ProjectileSpeed,
        &ProjectileTarget,
        &mut AttackDuration,
        &ProjectileOrigin,
        &ProjectileControlPoint,
    ),(With<ArcaneMissile>,  Without<Enemy>)>,
    enemies: Query<&Transform,(With<Enemy>, Without<ArcaneMissile>)>,
    time: Res<Time>,
    // mut gizmos: Gizmos,
) {
    for (
        arcane_missile_entity,
        mut transform,
        mut sprite,
        mut velocity,
        speed,
        projectile_target,
        mut attack_duration,
        projectile_origin,
        projectile_control_point,
    ) in &mut arcane_missiles {
        if let Ok(enemy_transform) = enemies.get(projectile_target.0){
            attack_duration.timer.tick(time.delta());

            //debug
            // gizmos.circle_2d(Vec2::new(projectile_control_point.0.x,projectile_control_point.0.y),3.0, Color::WHITE);

            // let distance_enemy_player = enemy_transform.translation.distance(projectile_origin.0);
            // gizmos.circle_2d(Vec2::new(projectile_origin.x,projectile_origin.y),distance_enemy_player/2.0 + 10.0, Color::PURPLE);
            // gizmos.circle_2d(Vec2::new(enemy_transform.translation.x,enemy_transform.translation.y),distance_enemy_player/2.0 + 10.0, Color::RED);


            let direction = (transform.translation.truncate()
                - enemy_transform.translation.truncate())
                .normalize();
            sprite.flip_x = direction.x < 0.0;

            velocity.x = direction.x * time.delta_seconds() * speed.0;
            velocity.y = direction.y * time.delta_seconds() * speed.0;



            let t =  attack_duration.timer.elapsed().as_millis() as f32 / attack_duration.timer.duration().as_millis()  as f32;
            transform.translation = simple_bezier(projectile_origin.0, projectile_control_point.0, enemy_transform.translation, t);
        }
        else {
            // delete projectile
            commands.entity(arcane_missile_entity).despawn_recursive();
        }


    }
}



// fn apply_arcane_missile_velocity(
//     mut arcane_missiles: Query<(&mut Transform, &mut ProjectileVelocity)>,
// ) {
//     for (mut transform, mut velocity) in &mut arcane_missiles {
//         transform.translation -= velocity.extend(0.0);
//     }
// }
