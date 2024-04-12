use crate::components::*;
use bevy::prelude::*;
use bevy::time::Stopwatch;

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
    
    fn build(&self, app: &mut App) {
        app.insert_resource(WaveManagerGlobalTime {
            global_time: Stopwatch::new(),
        })
        app.add_systems(Update, (waves_manager_tick, waves_spawn));
    }
}

fn waves_manager_tick(mut commands: Commands, mut waves: Query<&mut WaveManager>, time: Res<Time>) {
    for mut wave_manager in &mut waves {
        wave_manager.start_delay.tick(time.delta());
        wave_manager.end_delay.tick(time.delta());

        if wave_manager.start_delay.just_finished() {
            for wave_prefab in wave_manager.waves_prefab.clone() {
                let wave_id = commands.spawn(wave_prefab.clone()).id();
                wave_manager.waves.push(wave_id);
            }
        }
        if wave_manager.end_delay.just_finished() {
            for wave in wave_manager.waves.clone() {
                commands.entity(wave).despawn_recursive();
            }
        }
    }
}

fn waves_spawn(
    mut waves: Query<&mut Wave>,
    time: Res<Time>,
    mut spawn_enemy: EventWriter<SpawnEnemy>,
) {
    for mut wave in &mut waves {
        wave.delay_between_spawn.tick(time.delta());
        if !wave.delay_between_spawn.just_finished() {
            continue;
        }

        for _ in 0..wave.amount_per_timer_trigger {
            spawn_enemy.send(SpawnEnemy {
                enemy_types: wave.enemy_type,
            });
        }

        

        // match wave.enemy_type {
        //     EnemyTypes::Bat => {
        //
        //     }
        //     EnemyTypes::Bee => {}
        //     EnemyTypes::Golem => {}
        //     EnemyTypes::Rabbit => {}
        //     EnemyTypes::Skull => {}
        // }
    }
}
