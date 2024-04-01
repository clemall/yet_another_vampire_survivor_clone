use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::weapons::generic_systems::start_reload_attack_spawner;


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
                spawn_claw_attack.after(start_reload_attack_spawner),
                claw_attack_animation_and_collider,
            ).run_if(in_state(GameState::Gameplay))
        );
    }
}

fn run_if_claw_present(
     player_weapons: Res<PlayerWeapons>,
     weapon: Query<(), With<ClawSpawner>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::Claw) && weapon.is_empty()
}

fn setup_claw_spawner(mut commands: Commands){
    commands.spawn((
        ClawSpawner,
        DelayBetweenAttacks {
            timer:Timer::from_seconds(0.6, TimerMode::Repeating),
        },
        AttackAmmo{
            size: 2,
            amount: 2,
            reload_time: 2.0,
        },
        ProjectileBendLeftOrRight(true),
        Name::new("Claw Spawner"),
    ));
}

fn spawn_claw_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawner: Query<(
        &mut DelayBetweenAttacks,
        &mut AttackAmmo,
        &mut ProjectileBendLeftOrRight
    ), (With<ClawSpawner>, Without<AttackReloadDuration>)>,
    mut player: Query<&Transform, With<Player>>,
    time: Res<Time>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
){
    let player_transform= player.single_mut();

    if let Ok(
        (mut attack_timer, mut attack_ammo, mut projectile_orientation)
    ) = spawner.get_single_mut(){
        attack_timer.timer.tick(time.delta());

        if attack_timer.timer.just_finished() {
           let texture = asset_server.load("claw.png");
            let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 48.0), 2, 1, Option::from(Vec2::new(1.0, 1.0)), None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            let mut pos_x = player_transform.translation.x;

            let is_flip = match projectile_orientation.0 {
                true => {
                     pos_x -= CLAWS_POSITION_X;
                    true
                }
                false => {
                    pos_x += CLAWS_POSITION_X;
                    false
                }
            };

            **projectile_orientation = !projectile_orientation.0;

            attack_ammo.amount -= 1;
            
      

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
                Sensor)
            ).insert((
                Collider::cuboid(48.0/2.0, 48.0/2.0),
                ProjectileBundleCollider::default(),
                ProjectileLifetime {
                    timer:Timer::from_seconds(0.3, TimerMode::Once),
                },
                AlreadyHitEnemies{seen:Vec::new()},
                ProjectileDamage(5.0),
                ProjectileImpulse(2000.0),
                Claw,
                Projectile,
                Name::new("Claw Attack"),
            ));
        }
    }
}



fn claw_attack_animation_and_collider(
    mut claws: Query<(&mut Transform, &ProjectileLifetime), With<Claw>>,
) {
    for (mut tranform, attack_duration)  in &mut claws {
        // transform claw attack
        tranform.scale.x = (attack_duration.timer.fraction() * 0.2) + 0.8;
        tranform.scale.y = (attack_duration.timer.fraction() * 0.2) + 0.8;

    }
}

