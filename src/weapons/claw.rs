use std::time::Duration;
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy_pixel_camera::{PixelViewport, PixelZoom};
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::enemies::enemy::{damage_enemy, enemy_death_check};


const CLAWS_POSITION_X:f32 = 28.0;
pub struct WeaponClawPlugin;

impl Plugin for WeaponClawPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_claw_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_claw_present)
            )
        );
        app.add_systems(Update,(
            spawn_claw_attack,
            claw_attack_duration_tick,
            claw_attack_animation_and_collider,
            claw_damage.before(enemy_death_check),
            claw_attack_despawn).run_if(in_state(GameState::Gameplay))
        );
    }
}

fn run_if_claw_present(
     mut player_weapons: Res<PlayerWeapons>,
     weapon: Query<(), With<ClawSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::Claw) && weapon.is_empty()
}

fn setup_claw_spawner(mut commands: Commands){
    let mut timer = Timer::from_seconds(2.0, TimerMode::Repeating);
    timer.set_elapsed(Duration::from_secs(1));

    commands.spawn((
        ClawSpawner,
        AttackTimer{
            timer:timer,
        },
        Name::new("Claw Spawner"),
    ));
}

fn spawn_claw_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut claw_spawner: Query<(&mut ClawSpawner, &mut AttackTimer,)>,
    mut player: Query<(&Transform, &mut Player)>,
    time: Res<Time>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
){
    let (player_transform, player) = player.single_mut();
    // let (mut spawner, mut attack_timer) = claw_spawner.get_single_mut();

    if let Ok((mut spawner, mut attack_timer)) = claw_spawner.get_single_mut(){
        attack_timer.timer.tick(time.delta());


        if attack_timer.timer.just_finished() {
            factory_claw(&mut commands, &asset_server, &mut texture_atlas_layouts, &player_transform, Facing::Left);
            factory_claw(&mut commands, &asset_server, &mut texture_atlas_layouts, &player_transform, Facing::Right);
        }
    }
}

fn factory_claw(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_transform:&Transform,
    facing: Facing,
) {
    let texture = asset_server.load("claw.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 48.0), 2, 1, Option::from(Vec2::new(1.0, 1.0)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut pos_x = player_transform.translation.x;
    let mut is_flip = false;
    match facing {
        Facing::Left => {
            pos_x -= CLAWS_POSITION_X;
            is_flip = true;
        }
        Facing::Right => {
            pos_x += CLAWS_POSITION_X;
            is_flip = false;
        }
    }

    commands.spawn((
        SpriteBundle {
            texture,
            // transform: Transform::from_xyz(pos_x, player_transform.translation.y, 1.0),
            transform: Transform{
                translation: Vec3::new(pos_x, player_transform.translation.y, 1.0),
                // scale: Vec3::new(0.0, 0.0, 0.0),
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
        AnimationIndices { first: 0, last: 1, is_repeating: false },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Sensor,
        Collider::cuboid(48.0/2.0, 48.0/2.0),
        AttackDuration{
            timer:Timer::from_seconds(0.3, TimerMode::Once),
        },
        Claw{
            damage: 10.0,
        },
        AlreadyHitEnemies{seen:Vec::new()},
        Name::new("Claw Attack"),
    ));
}

fn claw_attack_duration_tick(
    mut claws: Query<&mut AttackDuration, With<Claw>>,
    time: Res<Time>,
) {
     for ( mut attack_duration) in &mut claws {
         attack_duration.timer.tick(time.delta());
     }
}

fn claw_attack_animation_and_collider(
    mut claws: Query<(&mut Transform, &mut Sprite, &mut Collider, &mut AttackDuration), With<Claw>>,
    time: Res<Time>,
) {
    for (mut tranform, mut sprite,mut collider, mut attack_duration)  in &mut claws {
        // transform claw attack
        tranform.scale.x = (attack_duration.timer.fraction() * 0.2) + 0.8;
        tranform.scale.y = (attack_duration.timer.fraction() * 0.2) + 0.8;
        // collider claw attack
        // let new_scale = Vec2::new(
        //     (attack_duration.timer.fraction() * 1.1),
        //     (attack_duration.timer.fraction() * 1.2)
        // );
        // collider.set_scale(new_scale, 1);
    }
}

fn claw_attack_despawn(
    mut commands: Commands,
    mut claws: Query<(Entity, &mut AttackDuration), With<Claw>>,
    time: Res<Time>,
) {
    for (entity, mut attack_duration)  in &mut claws {
        if attack_duration.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn claw_damage(
    mut commands: Commands,
    mut claws: Query<(
        &Collider,
        &GlobalTransform,
        &mut Claw,
        &mut AlreadyHitEnemies,
    ), Without<ColliderDisabled>>,
    mut enemy: Query<(&mut Health, &Transform), With<Enemy>>,
    mut player: Query<(&Transform, &mut Player)>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (collider, transform, mut claw, mut seen_enemies) in &mut claws {
        // let filter = QueryFilter::new()
        //     .predicate(&|handle: Entity| seen_enemies.seen.contains(&handle.index()));

        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok((mut health, transform)) = enemy.get_mut(entity) {
                    if !seen_enemies.seen.contains(&entity.index()){
                        damage_enemy(&mut commands,  entity,health, transform, claw.damage);

                        seen_enemies.seen.push(entity.index());

                        let (player_transform, player) = player.single_mut();
                        let mut direction:Vec2 = (transform.translation.truncate() -player_transform.translation.truncate());
                        commands.entity(entity).try_insert(ExternalImpulse   {
                            impulse: direction.normalize() * 2000.0,
                            torque_impulse: 0.0,
                        },);
                    }
                }
                true
            },
        );
    }
}

