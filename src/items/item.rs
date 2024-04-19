use crate::components::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(LootTable {
        //     weighted_rarity: [
        //         (Rarity::Common, 300),
        //         (Rarity::Uncommon, 100),
        //         (Rarity::Rare, 30),
        //         (Rarity::Epic, 10),
        //         (Rarity::Legendary, 2),
        //         (Rarity::Cursed, 0), // Always offered once
        //         (Rarity::Unique, 1),
        //     ],
        //     item_by_rarity: HashMap::from([
        //         (
        //             Rarity::Common,
        //             vec![
        //                 ItemsTypes::MaxHealth,
        //                 ItemsTypes::Recovery,
        //                 ItemsTypes::MoveSpeed,
        //                 ItemsTypes::Magnet,
        //                 ItemsTypes::Power,
        //                 ItemsTypes::Magnet,
        //                 ItemsTypes::Area,
        //             ],
        //         ),
        //         (
        //             Rarity::Uncommon,
        //             vec![
        //                 ItemsTypes::MaxHealth,
        //                 ItemsTypes::Recovery,
        //                 ItemsTypes::MoveSpeed,
        //                 ItemsTypes::Magnet,
        //                 ItemsTypes::Power,
        //                 ItemsTypes::Magnet,
        //                 ItemsTypes::Area,
        //             ],
        //         ),
        //         (
        //             Rarity::Rare,
        //             vec![
        //                 ItemsTypes::MaxHealth,
        //                 ItemsTypes::Recovery,
        //                 ItemsTypes::MoveSpeed,
        //                 ItemsTypes::Magnet,
        //                 ItemsTypes::Power,
        //                 ItemsTypes::Magnet,
        //                 ItemsTypes::Area,
        //             ],
        //         ),
        //         (
        //             Rarity::Epic,
        //             vec![
        //                 ItemsTypes::MaxHealth,
        //                 ItemsTypes::Recovery,
        //                 ItemsTypes::MoveSpeed,
        //                 ItemsTypes::Magnet,
        //                 ItemsTypes::Power,
        //                 ItemsTypes::Magnet,
        //                 ItemsTypes::Area,
        //             ],
        //         ),
        //         (
        //             Rarity::Legendary,
        //             vec![
        //                 ItemsTypes::MaxHealth,
        //                 ItemsTypes::Recovery,
        //                 ItemsTypes::MoveSpeed,
        //                 ItemsTypes::Magnet,
        //                 ItemsTypes::Power,
        //                 ItemsTypes::Magnet,
        //                 ItemsTypes::Area,
        //             ],
        //         ),
        //         (Rarity::Cursed, vec![ItemsTypes::WipCurseDamage]),
        //         (Rarity::Unique, vec![ItemsTypes::WipUniqueDamage]),
        //     ]),
        // });
        
        app.add_systems(
            Startup,
            setup_loot_table,
        );
        
        app.add_systems(
            Update,
            (
                trigger_item
            )
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn setup_loot_table(mut commands: Commands,){
    let data = fs::read_to_string("assets/items.ron").unwrap();
    let item_resource: ItemsResource = ron::from_str(&data).unwrap();
    commands.insert_resource(item_resource.clone());
    
    let mut loot_table = LootTable{ 
        weighted_rarity: item_resource.weighted_rarity.clone(),
        item_by_rarity:HashMap::from([
            (Rarity::Common, Vec::new()),
            (Rarity::Uncommon, Vec::new()),
            (Rarity::Rare, Vec::new()),
            (Rarity::Epic, Vec::new()),
            (Rarity::Legendary, Vec::new()),
            (Rarity::Cursed, Vec::new()),
            (Rarity::Unique, Vec::new()),
        ])
    };
    // populate loot_table.item_by_rarity
    // Will be a map of rarity with for each a list of item
    // the "item" is simply a key to the ItemsResource map of items
    for (item_key, item_data) in item_resource.items.clone() {
        for (rarity, _variation) in item_data.rarity_variations {
            match loot_table.item_by_rarity.get_mut(&rarity){
                Some(mut item_list) => {
                    item_list.push(item_key.clone());
                }
                None => {
                    println!("Check why the key is not found, should not be possible");
                }
            };
        }
    }
    
    
    commands.insert_resource(loot_table);
    
}

fn trigger_item(
    mut item_event: EventReader<ItemPickup>,
    mut player_stats: ResMut<PlayerInGameStats>,
    item_resource: Res<ItemsResource>,
) {
    for event in item_event.read() {
        let item_data = item_resource.items.get(&event.item_key).unwrap();
        let item_variation =  item_data.rarity_variations.get(&event.rarity).unwrap();
        
        match item_data.base_stat {
            PlayerBaseStatsType::MaxHealth => {
                player_stats.max_health += BASE_MAX_HEALTH * item_variation.value;
            }
            PlayerBaseStatsType::Recovery => {
                player_stats.recovery += BASE_RECOVERY * item_variation.value;
            }
            PlayerBaseStatsType::MoveSpeed => {
                player_stats.move_speed += BASE_MOVE_SPEED * item_variation.value;
            }
            PlayerBaseStatsType::Magnet => {
                player_stats.magnet += BASE_MAGNET * item_variation.value;
            }
            PlayerBaseStatsType::Power => {
                player_stats.power += BASE_POWER * item_variation.value;
            }
            PlayerBaseStatsType::Area => {
                player_stats.area += BASE_AREA * item_variation.value;
            }
            PlayerBaseStatsType::Luck => {
                // player_stats.luck += BASE_LUCK * item_variation.value;
            }
        }
        
    }
    
}
// 
// fn trigger_item_max_health(
//     mut item_pickup: EventReader<ItemPickup>,
//     mut player_stats: ResMut<PlayerInGameStats>,
// ) {
//     for event in item_pickup.read() {
//         if event.item_type != ItemsTypes::MaxHealth {
//             continue;
//         }
// 
//         match event.rarity {
//             Rarity::Common => {
//                 // increase max health by 10% of base value
//                 player_stats.max_health += BASE_MAX_HEALTH * 0.1;
//             }
//             Rarity::Uncommon => {
//                 player_stats.max_health += BASE_MAX_HEALTH * 0.15;
//             }
//             Rarity::Rare => {
//                 player_stats.max_health += BASE_MAX_HEALTH * 0.2;
//             }
//             Rarity::Epic => {
//                 player_stats.max_health += BASE_MAX_HEALTH * 0.3;
//             }
//             Rarity::Legendary => {
//                 player_stats.max_health += BASE_MAX_HEALTH * 0.5;
//             }
//             _ => {}
//         }
//     }
// }
// 
// fn trigger_item_magnet(
//     mut item_pickup: EventReader<ItemPickup>,
//     mut player_stats: ResMut<PlayerInGameStats>,
// ) {
//     for event in item_pickup.read() {
//         if event.item_type != ItemsTypes::Magnet {
//             continue;
//         }
// 
//         match event.rarity {
//             Rarity::Common => {
//                 player_stats.magnet += BASE_MAGNET * 0.1;
//             }
//             Rarity::Uncommon => {
//                 player_stats.magnet += BASE_MAGNET * 0.15;
//             }
//             Rarity::Rare => {
//                 player_stats.magnet += BASE_MAGNET * 0.2;
//             }
//             Rarity::Epic => {
//                 player_stats.magnet += BASE_MAGNET * 0.3;
//             }
//             Rarity::Legendary => {
//                 player_stats.magnet += BASE_MAGNET * 0.5;
//             }
//             _ => {}
//         }
//     }
// }
// 
// fn trigger_item_move_speed(
//     mut item_pickup: EventReader<ItemPickup>,
//     mut player_stats: ResMut<PlayerInGameStats>,
// ) {
//     for event in item_pickup.read() {
//         if event.item_type != ItemsTypes::MoveSpeed {
//             continue;
//         }
// 
//         match event.rarity {
//             Rarity::Common => {
//                 player_stats.move_speed += BASE_MOVE_SPEED * 0.05;
//             }
//             Rarity::Uncommon => {
//                 player_stats.move_speed += BASE_MOVE_SPEED * 0.07;
//             }
//             Rarity::Rare => {
//                 player_stats.move_speed += BASE_MOVE_SPEED * 0.1;
//             }
//             Rarity::Epic => {
//                 player_stats.move_speed += BASE_MOVE_SPEED * 0.15;
//             }
//             Rarity::Legendary => {
//                 player_stats.move_speed += BASE_MOVE_SPEED * 0.2;
//             }
//             _ => {}
//         }
//     }
// }
// 
// fn trigger_item_power(
//     mut item_pickup: EventReader<ItemPickup>,
//     mut player_stats: ResMut<PlayerInGameStats>,
// ) {
//     for event in item_pickup.read() {
//         if event.item_type != ItemsTypes::Power {
//             continue;
//         }
// 
//         match event.rarity {
//             Rarity::Common => {
//                 player_stats.power += BASE_POWER * 0.05;
//             }
//             Rarity::Uncommon => {
//                 player_stats.power += BASE_POWER * 0.1;
//             }
//             Rarity::Rare => {
//                 player_stats.power += BASE_POWER * 0.2;
//             }
//             Rarity::Epic => {
//                 player_stats.power += BASE_POWER * 0.3;
//             }
//             Rarity::Legendary => {
//                 player_stats.power += BASE_POWER * 0.5;
//             }
//             _ => {}
//         }
//     }
// }
// 
// fn trigger_item_area(
//     mut item_pickup: EventReader<ItemPickup>,
//     mut player_stats: ResMut<PlayerInGameStats>,
// ) {
//     for event in item_pickup.read() {
//         if event.item_type != ItemsTypes::Area {
//             continue;
//         }
// 
//         match event.rarity {
//             Rarity::Common => {
//                 player_stats.area += BASE_POWER * 0.05;
//             }
//             Rarity::Uncommon => {
//                 player_stats.area += BASE_POWER * 0.1;
//             }
//             Rarity::Rare => {
//                 player_stats.area += BASE_POWER * 0.2;
//             }
//             Rarity::Epic => {
//                 player_stats.area += BASE_POWER * 0.3;
//             }
//             Rarity::Legendary => {
//                 player_stats.area += BASE_POWER * 0.5;
//             }
//             _ => {}
//         }
//     }
// }
// 
// fn trigger_item_recovery(
//     mut item_pickup: EventReader<ItemPickup>,
//     mut player_stats: ResMut<PlayerInGameStats>,
// ) {
//     for event in item_pickup.read() {
//         if event.item_type != ItemsTypes::Recovery {
//             continue;
//         }
// 
//         match event.rarity {
//             Rarity::Common => {
//                 player_stats.recovery += BASE_RECOVERY;
//             }
//             Rarity::Uncommon => {
//                 player_stats.recovery += BASE_RECOVERY * 2.0;
//             }
//             Rarity::Rare => {
//                 player_stats.recovery += BASE_RECOVERY * 3.0;
//             }
//             Rarity::Epic => {
//                 player_stats.recovery += BASE_RECOVERY * 4.0;
//             }
//             Rarity::Legendary => {
//                 player_stats.recovery += BASE_RECOVERY * 5.0;
//             }
//             _ => {}
//         }
//     }
// }
// 
// fn trigger_item_curse_damage(
//     mut item_pickup: EventReader<ItemPickup>,
//     mut player_stats: ResMut<PlayerInGameStats>,
// ) {
//     for event in item_pickup.read() {
//         if event.item_type != ItemsTypes::WipCurseDamage {
//             continue;
//         }
// 
//         // double damage
//         player_stats.power += BASE_POWER * 1.0;
//         // but make you slow
//         player_stats.move_speed -= BASE_MOVE_SPEED * 0.5;
//     }
// }
// 
// fn trigger_item_unique_damage(
//     mut item_pickup: EventReader<ItemPickup>,
//     mut player_stats: ResMut<PlayerInGameStats>,
// ) {
//     for event in item_pickup.read() {
//         if event.item_type != ItemsTypes::WipUniqueDamage {
//             continue;
//         }
// 
//         player_stats.power += BASE_POWER * 1.0;
//     }
// }
