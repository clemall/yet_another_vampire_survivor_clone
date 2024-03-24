use std::time::Duration;
use bevy_pixel_camera::{PixelViewport, PixelZoom};
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::enemies::enemy::{damage_enemy, enemy_death_check};
use bevy::{
    math::{cubic_splines::CubicCurve, vec3},
    prelude::*,
};
use bevy_inspector_egui::egui::debug_text::print;

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

fn run_if_weapon_is_added(
     mut player_weapons: Res<PlayerWeapons>,
     weapon: Query<(), With<ArcaneMissileSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::ArcaneMissile) && weapon.is_empty()
}


fn run_if_arcane_missile_present(
     mut player_weapons: Res<PlayerWeapons>,
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
    time: Res<Time>,
){
    if let Ok((entity, mut attack_ammo)) = arcane_missile_spawner.get_single_mut(){
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
    mut player: Query<(&Transform, &mut Player)>,
    mut arcane_missile_spawner: Query<(&mut AttackTimer, &mut AttackAmmo),(With<ArcaneMissileSpawner>, Without<AttackReload>)>,
    enemies: Query<(Entity, &Transform),With<Enemy>>,
    mut projectile_offset_bool: ResMut<ProjectileOffsetGoesLeft>,
    time: Res<Time>,
){
    let (player_transform, player) = player.single_mut();

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
                    let mut control_point = Vec3::new(player_transform.translation.x ,player_transform.translation.y , 0.0 );
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

                    if projectile_offset_bool.0{
                        control_point = control_point_1;
                    }
                    else {
                        control_point = control_point_2;
                    }



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

            // match closed_enemy {
            //     Some(closed_enemy) => {
            //         factory_arcane_missile(&mut commands,
            //                                &asset_server,
            //                                &mut texture_atlas_layouts,
            //                                &player_transform,
            //                                &closed_enemy,
            //                                &enemies,
            //                                &mut projectile_offset_bool);
            //     }
            //     None => {}
            // }

        }
    }
}


fn arcane_missile_damage(
    mut commands: Commands,
    mut arcane_missiles: Query<(
        Entity,
        &Collider,
        &GlobalTransform,
        &mut ArcaneMissile,
    ), Without<ColliderDisabled>>,
    mut enemy: Query<(&mut Health, &Transform), With<Enemy>>,
    mut player: Query<(&Transform, &mut Player)>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (arcane_missile_entity, collider, transform, mut arcane_missile) in &mut arcane_missiles {

        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok((mut health, transform)) = enemy.get_mut(entity) {
                    damage_enemy(&mut commands,entity,  health, transform, arcane_missile.damage);

                    let (player_transform, player) = player.single_mut();
                    let mut direction:Vec2 = (transform.translation.truncate() -player_transform.translation.truncate());
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
        &mut ProjectileSpeed,
        &mut ProjectileTarget,
        &mut AttackDuration,
        &ProjectileOrigin,
        &mut ProjectileControlPoint,
    ),(With<ArcaneMissile>,  Without<Enemy>)>,
    enemies: Query<(Entity, &Transform),(With<Enemy>, Without<ArcaneMissile>)>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    for (
        arcane_missile_entity,
        mut transform,
        mut sprite,
        mut velocity,
        mut speed,
        mut projectile_target,
        mut attack_duration,
        projectile_origin,
        mut projectile_control_point,
    ) in &mut arcane_missiles {
        if let Ok((entity, enemy_transform)) = enemies.get(projectile_target.0){
            attack_duration.timer.tick(time.delta());

            //debug
            // gizmos.circle_2d(Vec2::new(projectile_control_point.0.x,projectile_control_point.0.y),3.0, Color::WHITE);

            // let distance_enemy_player = enemy_transform.translation.distance(projectile_origin.0);
            // gizmos.circle_2d(Vec2::new(projectile_origin.x,projectile_origin.y),distance_enemy_player/2.0 + 10.0, Color::PURPLE);
            // gizmos.circle_2d(Vec2::new(enemy_transform.translation.x,enemy_transform.translation.y),distance_enemy_player/2.0 + 10.0, Color::RED);


            let mut direction = (transform.translation.truncate()
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


fn simple_bezier(a: Vec3, b: Vec3, c: Vec3, t: f32) -> Vec3{
    let ab = a.lerp(b, t);
    let bc = b.lerp(c, t);
    ab.lerp(bc, t)
}

// Find the points where the two circles intersect.
fn find_circle_circle_intersections(c0: Vec3, r0: f32, c1: Vec3, r1: f32) -> (Vec3, Vec3){
    // Find the distance between the centers.
    let dx= c0.x - c1.x;
    let dy = c0.y - c1.y;
    let dist = (dx * dx + dy * dy).sqrt();

    if ((dist - (r0 + r1)).abs() < 0.00001)
    {
        let intersection1 = c0.lerp(c1, r0 / (r0 + r1));
        let intersection2 = intersection1;
        return (intersection1, intersection2)
    }

    // See how many solutions there are.
    if (dist > r0 + r1)
    {
        // No solutions, the circles are too far apart.
        let intersection1 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        let intersection2 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        return (intersection1, intersection2)
    }
    else if (dist < (r0 - r1).abs())
    {
        // No solutions, one circle contains the other.
        let intersection1 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        let intersection2 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        return (intersection1, intersection2)
    }
    else if ((dist == 0.0) && (r0 == r1))
    {
        // No solutions, the circles coincide.
        let intersection1 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        let intersection2 = Vec3::new(f32::NAN, f32::NAN, 0.0);
        return (intersection1, intersection2)
    }
    else
    {
        // Find a and h.
        let a = (r0 * r0 -
                    r1 * r1 + dist * dist) / (2.0 * dist);
        let h = (r0 * r0 - a * a).sqrt();

        // Find P2.
        let cx2 = c0.x + a * (c1.x - c0.x) / dist;
        let cy2 = c0.y + a * (c1.y - c0.y) / dist;

        // Get the points P3.
        let intersection1 = Vec3::new(
            (cx2 + h * (c1.y - c0.y) / dist),
            (cy2 - h * (c1.x - c0.x) / dist), 0.0);
        let intersection2 = Vec3::new(
            (cx2 - h * (c1.y - c0.y) / dist),
            (cy2 + h * (c1.x - c0.x) / dist), 0.0);

         return (intersection1, intersection2)
    }
}


// fn apply_arcane_missile_velocity(
//     mut arcane_missiles: Query<(&mut Transform, &mut ProjectileVelocity)>,
// ) {
//     for (mut transform, mut velocity) in &mut arcane_missiles {
//         transform.translation -= velocity.extend(0.0);
//     }
// }
