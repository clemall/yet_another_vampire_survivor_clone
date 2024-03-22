use std::time::Duration;
use bevy::prelude::*;
use bevy_pixel_camera::{PixelViewport, PixelZoom};
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::enemies::enemy::{damage_enemy, enemy_death_check};


pub struct ArcaneMissilePlugin;

impl Plugin for ArcaneMissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_arcane_missile_spawner.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_arcane_missile_present)
            )
        );

        app.add_systems(Update, (spawn_arcane_missile_attack, compute_arcane_missile_velocity, apply_arcane_missile_velocity));
    }
}

fn run_if_arcane_missile_present(
     mut player_weapons: Res<PlayerWeapons>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::ArcaneMissile)
}



fn setup_arcane_missile_spawner(mut commands: Commands){
    let mut timer = Timer::from_seconds(2.0, TimerMode::Repeating);
    timer.set_elapsed(Duration::from_secs(1));

    commands.spawn((
        ArcaneMissileSpawner,
        AttackTimer{
            timer
        },
        Name::new("Arcane missile Spawner"),
    ));
}



fn spawn_arcane_missile_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut arcane_missile_spawner: Query<(&mut ArcaneMissileSpawner, &mut AttackTimer,)>,
    mut player: Query<(&Transform, &mut Player)>,
    time: Res<Time>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    enemies: Query<(Entity, &Transform),With<Enemy>>
){
    let (player_transform, player) = player.single_mut();

    if let Ok((mut spawner, mut attack_timer)) = arcane_missile_spawner.get_single_mut(){
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

            match closed_enemy {
                Some(closed_enemy) => {
                    factory_arcane_missile(&mut commands, &asset_server, &mut texture_atlas_layouts, &player_transform, &closed_enemy, &enemies);
                    factory_arcane_missile(&mut commands, &asset_server, &mut texture_atlas_layouts, &player_transform, &closed_enemy, &enemies);
                    factory_arcane_missile(&mut commands, &asset_server, &mut texture_atlas_layouts, &player_transform, &closed_enemy, &enemies);
                }
                None => {}
            }

        }
    }
}



fn factory_arcane_missile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_transform:&Transform,
    enemy_entity:&Entity,
    enemies: &Query<(Entity, &Transform),With<Enemy>>
) {
    let texture = asset_server.load("arcane_missile.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 19.0), 2, 1, Option::from(Vec2::new(1.0, 0.0)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    if let Ok((entity, transform)) = enemies.get(*enemy_entity) {
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
                damage: 20.0,
            },
            ProjectileTarget(entity),
            ProjectileVelocity(Vec2::new(0.0, 0.0)),
            ProjectileSpeed(30.0),
            Name::new("Arcane missile Attack"),
        ));
    }


}




fn compute_arcane_missile_velocity(
    mut arcane_missiles: Query<(&mut Transform, &mut Sprite, &mut ProjectileVelocity, &mut ProjectileSpeed, &mut ProjectileTarget),(With<ArcaneMissile>,  Without<Enemy>)>,
    enemies: Query<(Entity, &Transform),(With<Enemy>, Without<ArcaneMissile>)>,
    time: Res<Time>,
) {
    for (mut transform, mut sprite, mut velocity,  mut speed, mut projectile_target) in &mut arcane_missiles {
        if let Ok((entity, enemy_transform)) = enemies.get(projectile_target.0){

            let mut direction = (transform.translation.truncate()
                - enemy_transform.translation.truncate())
                .normalize();
            sprite.flip_x = direction.x < 0.0;

            velocity.x = direction.x * time.delta_seconds() * speed.0;
            velocity.y = direction.y * time.delta_seconds() * speed.0;
        }


    }
}




fn apply_arcane_missile_velocity(
    mut arcane_missiles: Query<(&mut Transform, &mut ProjectileVelocity)>,
) {
    for (mut transform, mut velocity) in &mut arcane_missiles {
        transform.translation -= velocity.extend(0.0);
    }
}
