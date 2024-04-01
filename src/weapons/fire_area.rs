use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;


pub struct WeaponFireAreaPlugin;

impl Plugin for WeaponFireAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_fire_area.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_fire_area_present)
            )
        );
    }
}

fn run_if_fire_area_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<FireArea>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::FireArea) && weapon.is_empty()
}

pub fn setup_fire_area(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_transform: Query<&Transform, With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
){
    let player_transform = player_transform.single();
    let texture = asset_server.load("fire-area.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 48.0), 3, 1, Option::from(Vec2::new(1.0, 0.0)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform{
                translation: Vec3::new(player_transform.translation.x, player_transform.translation.y, 1.0),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        AnimationIndices { first: 0, last: 2, is_repeating: true },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Sensor,
        Collider::ball(48.0/2.0),
        ProjectileBundleCollider::default(),
    )).insert((
        FireAreaSpawner,
        FireArea,
        Projectile,
        ProjectileImpulse(150.0),
        ProjectileDamage(10.0),
        ProjectileFollowPlayer,
        ProjectileTimeBetweenDamage {
            timer:Timer::from_seconds(0.33, TimerMode::Repeating),
        },
        Name::new("Fire area Attack")));
}

//
// fn fire_area_damage(
//     mut commands: Commands,
//     mut fire_areas: Query<(
//         &Collider,
//         &GlobalTransform,
//         &FireArea,
//         &mut AttackDelayBetweenAttacks,
//     ), Without<ColliderDisabled>>,
//     mut enemy: Query<(&mut Health, &Transform), With<Enemy>>,
//     mut player: Query<&Transform, With<Player>>,
//     rapier_context: Res<RapierContext>,
//     time: Res<Time>,
// ) {
//     for (collider, transform, fire_area, mut attack_timer) in &mut fire_areas {
//         attack_timer.timer.tick(time.delta());
//
//         if attack_timer.timer.just_finished() {
//             rapier_context.intersections_with_shape(
//                 transform.translation().truncate(),
//                 0.0,
//                 collider,
//                 QueryFilter::new(),
//                 |entity| {
//                     if let Ok((health, transform)) = enemy.get_mut(entity) {
//                         damage_enemy(&mut commands,entity, health, transform, fire_area.damage);
//
//                         let player_transform = player.single_mut();
//                         let direction:Vec2 = transform.translation.truncate() -player_transform.translation.truncate();
//                         commands.entity(entity).try_insert(ExternalImpulse   {
//                             impulse: direction.normalize() * 200.0,
//                             torque_impulse: 0.0,
//                         },);
//
//                     }
//                     true
//                 },
//             );
//         }
//
//     }
// }

