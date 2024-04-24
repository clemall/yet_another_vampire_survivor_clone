use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct FireAreaSpawner;

#[derive(Component)]
pub struct FireArea;
pub struct WeaponFireAreaPlugin;

impl Plugin for WeaponFireAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_fire_area.run_if(
                resource_exists_and_changed::<PlayerWeapons>.and_then(run_if_fire_area_present),
            ),
        );
        app.add_systems(
            Update,
            (spawn_fire_area_attack,).run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn run_if_fire_area_present(
    player_weapons: Res<PlayerWeapons>,
    weapon: Query<(), With<FireArea>>,
) -> bool {
    player_weapons.weapons.contains(&WeaponsTypes::FireArea) && weapon.is_empty()
}

fn setup_fire_area(mut commands: Commands, _player_stats: Res<PlayerInGameStats>) {
    commands.spawn((FireAreaSpawner, CanAttack, Name::new("Fire area Spawner")));
}

pub fn spawn_fire_area_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_transform: Query<&Transform, With<Player>>,
    spawner: Query<Entity, (With<FireAreaSpawner>, With<CanAttack>)>,
    player_stats: Res<PlayerInGameStats>,
) {
    let player_transform = player_transform.single();

    if let Ok(spawner_entity) = spawner.get_single() {
        let texture = asset_server.load("fire-area.png");
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(48.0, 48.0),
            3,
            1,
            Option::from(Vec2::new(1.0, 0.0)),
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        commands.entity(spawner_entity).remove::<CanAttack>();

        commands
            .spawn((
                SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: Vec3::new(
                            player_transform.translation.x,
                            player_transform.translation.y,
                            1.0,
                        ),
                        scale: Vec3::splat(player_stats.area),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
                AnimationIndices {
                    first: 0,
                    last: 2,
                    is_repeating: true,
                },
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                Sensor,
                Collider::ball(48.0 / 2.0),
                ProjectileBundleCollider::default(),
            ))
            .insert((
                FireArea,
                Projectile,
                ProjectileFromWeapon(WeaponsTypes::FireArea),
                ProjectileImpulse(150.0),
                ProjectileDamage(10.0),
                ProjectilePositionOnPlayer,
                ProjectileTimeBetweenDamage {
                    timer: Timer::from_seconds(0.33, TimerMode::Repeating),
                },
                Name::new("Fire area Attack"),
            ));
    }
}
