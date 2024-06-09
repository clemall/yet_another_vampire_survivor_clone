use crate::components::{GameState, OnUpgradePickup, PlayerUpgradeWeapons};
use crate::weapons::arcane_missile::ArcaneMissilePlugin;
use crate::weapons::bouncing_ball::BouncingBallPlugin;
use crate::weapons::chain_lightning::ChainLightningPlugin;
use crate::weapons::claw::WeaponClawPlugin;
use crate::weapons::fire_area::WeaponFireAreaPlugin;
use crate::weapons::fire_boots::FireBootsPlugin;
use crate::weapons::light_sword::LightSwordsPlugin;
use crate::weapons::projectiles::ProjectilePlugin;
use crate::weapons::shuriken::ShurikenPlugin;
use crate::weapons::slow_dome::SlowDomePlugin;
use bevy::prelude::*;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProjectilePlugin);

        app.add_plugins(WeaponClawPlugin);

        app.add_plugins(WeaponFireAreaPlugin);

        app.add_plugins(ArcaneMissilePlugin);

        app.add_plugins(ShurikenPlugin);

        app.add_plugins(ChainLightningPlugin);

        app.add_plugins(SlowDomePlugin);

        app.add_plugins(BouncingBallPlugin);

        app.add_plugins(FireBootsPlugin);

        app.add_plugins(LightSwordsPlugin);

        app.add_systems(
            Update,
            trigger_upgrade.run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn trigger_upgrade(
    mut upgrade_events: EventReader<OnUpgradePickup>,
    mut player_upgrade_weapons: ResMut<PlayerUpgradeWeapons>,
) {
    for event in upgrade_events.read() {
        player_upgrade_weapons.upgrades.push(event.upgrade);
        println!("-> Player upgrades: {:?}", player_upgrade_weapons.upgrades);
    }
}
