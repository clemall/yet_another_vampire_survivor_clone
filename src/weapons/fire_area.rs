use std::time::Duration;
use bevy::prelude::*;
use bevy_pixel_camera::{PixelViewport, PixelZoom};
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::enemies::enemy::{damage_enemy, enemy_death_check};


pub struct WeaponFireAreaPlugin;

impl Plugin for WeaponFireAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_fire_area.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_fire_area_present)
            )
        );
        app.add_systems(Update, (fire_area_follow_player, fire_area_damage.before(enemy_death_check)));
    }
}

fn run_if_fire_area_present(
     mut player_weapons: Res<PlayerWeapons>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::FireArea)
}

pub fn setup_fire_area(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_transform: Query<(&Transform), With<Player>>,
    time: Res<Time>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
){
    let player_transform = player_transform.single();
    let texture = asset_server.load("fire-area.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 48.0), 3, 1, Option::from(Vec2::new(1.0, 0.0)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut timer = Timer::from_seconds(0.33, TimerMode::Repeating);

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
        FireArea{
            damage: 5.0,
        },
        AttackTimer{
            timer,
        },
        Name::new("Fire area Attack"),
    ));
}

fn fire_area_follow_player(
    player: Query<&Transform, (With<Player>, Without<FireArea>)>,
    mut fire_area: Query<&mut Transform, (With<FireArea>, Without<Player>)>,
) {
    if let Ok(player) = player.get_single() {
        if let Ok(mut fire_area) = fire_area.get_single_mut() {
            fire_area.translation.x = player.translation.x;
            fire_area.translation.y = player.translation.y;
        }
    }
}


fn fire_area_damage(
    mut commands: Commands,
    mut fire_areas: Query<(
        &Collider,
        &GlobalTransform,
        &mut FireArea,
        &mut AttackTimer,
    ), Without<ColliderDisabled>>,
    mut enemy: Query<(&mut Health, &Transform), With<Enemy>>,
    mut player: Query<(&Transform, &mut Player)>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (collider, transform, mut fire_area, mut attack_timer) in &mut fire_areas {
        attack_timer.timer.tick(time.delta());

        if attack_timer.timer.just_finished() {
            rapier_context.intersections_with_shape(
                transform.translation().truncate(),
                0.0,
                collider,
                QueryFilter::new(),
                |entity| {
                    if let Ok((mut health, transform)) = enemy.get_mut(entity) {
                        damage_enemy(&mut commands,entity,  health, transform, fire_area.damage);

                        let (player_transform, player) = player.single_mut();
                        let mut direction:Vec2 = (transform.translation.truncate() -player_transform.translation.truncate());
                        commands.entity(entity).try_insert(ExternalImpulse   {
                            impulse: direction.normalize() * 200.0,
                            torque_impulse: 0.0,
                        },);

                    }
                    true
                },
            );
        }

    }
}

