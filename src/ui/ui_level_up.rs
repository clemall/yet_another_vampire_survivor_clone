use crate::components::*;
use crate::constants::{FONT, FONT_BOLD, SCREEN_WIDTH};
use bevy::prelude::*;
use rand::distributions::{Distribution, WeightedIndex};
use rand::seq::SliceRandom;
use rand::Rng;

pub struct UiLevelUpPlugin;

impl Plugin for UiLevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PlayerLevelUp), spawn_level_up_ui);
        app.add_systems(OnExit(GameState::PlayerLevelUp), despawn_level_up_ui);
        app.add_systems(
            Update,
            level_up_button_interaction.run_if(in_state(GameState::PlayerLevelUp)),
        );
    }
}

const HOVERED_BUTTON: Color = Color::rgb(0.80, 0.80, 0.80);
const NORMAL_BUTTON: Color = Color::rgb(1., 1., 1.);
const POPUP_BG_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.95);

fn despawn_level_up_ui(mut commands: Commands, ui: Query<Entity, With<LevelUpUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}

fn spawn_level_up_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    camera: Query<&Camera>,
    loot_table: Res<LootTable>,
    items_resource: Res<ItemsResource>,
) {
    let level_up_parent = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            LevelUpUI,
            Name::new("UI Group Level UP"),
        ))
        .id();

    let level_up_popup = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                position_type: PositionType::Relative,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            background_color: POPUP_BG_COLOR.into(),
            ..default()
        })
        .id();

    commands
        .entity(level_up_parent)
        .push_children(&[level_up_popup]);

    let camera = camera.single();
    let view_dimensions = camera.logical_viewport_size().unwrap();
    let ratio = view_dimensions.x / SCREEN_WIDTH as f32;

    let dist = WeightedIndex::new(loot_table.weighted_rarity.iter().map(|item| item.1)).unwrap();

    let mut item_to_offer = 5;
    while item_to_offer > 0 {
        let mut rarity = loot_table.weighted_rarity[dist.sample(&mut rand::thread_rng())].0;

        // has a small chance to be a cursed item
        if rand::thread_rng().gen_range(0.0..100.0) < 1.0 {
            rarity = loot_table.weighted_rarity[5].0;
        }

        let Some(item_key) = loot_table
            .item_by_rarity
            .get(&rarity)
            .unwrap()
            .choose(&mut rand::thread_rng())
        else {
            // unique rarity has a list of items that can be removed over time
            // To avoid the function to panic, we simply continue the loop and try again to pick
            // another rarity/item.
            continue;
        };

        // item is found, we can decrease the counter
        item_to_offer -= 1;

        let item_name = items_resource
            .items
            .get(&item_key.clone())
            .unwrap()
            .name
            .clone();

        let texture_atlas_index = items_resource
            .items
            .get(&item_key.clone())
            .unwrap()
            .texture_atlas_index
            .clone();

        let item_description = items_resource
            .items
            .get(&item_key.clone())
            .unwrap()
            .rarity_to_effects
            .get(&rarity)
            .unwrap()
            .description
            .clone();

        let card_item = card_ui_factory(
            &mut commands,
            &asset_server,
            &mut texture_atlas_layouts,
            ratio,
            &rarity,
            &*item_key,
            &*item_name,
            &*item_description,
            texture_atlas_index,
        );

        commands.entity(level_up_popup).push_children(&[card_item]);
    }
}

fn level_up_button_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonItemUpgrade), // UiImage
        (Changed<Interaction>, With<Button>),
    >,
    mut item_pickup: EventWriter<OnItemPickup>,
) {
    for (interaction, mut image, upgrade) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                item_pickup.send(OnItemPickup {
                    item_key: upgrade.item_key.clone(),
                    rarity: upgrade.rarity,
                });

                next_state.set(GameState::Gameplay);
            }
            Interaction::Hovered => {
                *image = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *image = NORMAL_BUTTON.into();
            }
        }
    }
}

fn card_ui_factory(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    ratio: f32,
    rarity: &Rarity,
    item_key: &str,
    item_name: &str,
    item_description: &str,
    texture_atlas_index: u32,
) -> Entity {
    let texture_icons = asset_server.load("design_items_ui.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(74.0, 61.0), 23, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let texture = match rarity {
        Rarity::Common => asset_server.load("item_ui_background_common.png"),
        Rarity::Uncommon => asset_server.load("item_ui_background_uncommon.png"),
        Rarity::Rare => asset_server.load("item_ui_background_rare.png"),
        Rarity::Epic => asset_server.load("item_ui_background_epic.png"),
        Rarity::Legendary => asset_server.load("item_ui_background_legendary.png"),
        Rarity::Cursed => asset_server.load("item_ui_background_curse.png"),
        Rarity::Unique => asset_server.load("item_ui_background_unique.png"),
    };

    let rarity_text_color = match rarity {
        Rarity::Common => Color::MAROON,
        Rarity::Uncommon => Color::YELLOW_GREEN,
        Rarity::Rare => Color::MIDNIGHT_BLUE,
        Rarity::Epic => Color::FUCHSIA,
        Rarity::Legendary => Color::GOLD,
        Rarity::Cursed => Color::CRIMSON,
        Rarity::Unique => Color::BLACK,
    };

    let card_item = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    width: Val::Px(80. * ratio),
                    height: Val::Px(112. * ratio),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: UiImage::new(texture),
                z_index: ZIndex::Global(10),
                ..default()
            },
            ButtonItemUpgrade {
                item_key: item_key.into(),
                rarity: *rarity,
            },
        ))
        .id();

    let card_icon = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    width: Val::Px(74. * ratio),
                    height: Val::Px(61. * ratio),
                    align_self: AlignSelf::Start,
                    margin: UiRect::top(Val::Px(11.0)),
                    ..default()
                },
                image: UiImage::new(texture_icons),
                z_index: ZIndex::Global(1),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: texture_atlas_index as usize,
            },
        ))
        .id();

    let item_name = commands
        .spawn(
            TextBundle::from_section(
                item_name,
                TextStyle {
                    font: asset_server.load(FONT_BOLD),
                    font_size: 22.0,
                    color: Color::BLACK,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(62.0),

                ..default()
            }),
        )
        .id();

    let item_description = commands
        .spawn(
            TextBundle::from_section(
                item_description,
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 16.0,
                    color: Color::BLACK,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(70.0),
                left: Val::Percent(7.0),
                width: Val::Percent(90.0),
                ..default()
            }),
        )
        .id();

    let item_rarity = commands
        .spawn(
            TextBundle::from_section(
                rarity.name(),
                TextStyle {
                    font: asset_server.load(FONT_BOLD),
                    font_size: 16.0,
                    color: rarity_text_color,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(90.0),
                left: Val::Percent(7.0),
                ..default()
            }),
        )
        .id();

    commands.entity(card_item).push_children(&[card_icon]);
    commands.entity(card_item).push_children(&[item_name]);
    commands
        .entity(card_item)
        .push_children(&[item_description]);
    commands.entity(card_item).push_children(&[item_rarity]);

    card_item
}
