use crate::components::*;
use crate::constants::{FONT, FONT_BOLD, SCREEN_WIDTH};
use bevy::prelude::*;
use rand::seq::SliceRandom;

pub struct UiChooseWeaponPlugin;

impl Plugin for UiChooseWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::PlayerChooseWeapon),
            spawn_choose_weapon_ui,
        );
        app.add_systems(
            OnExit(GameState::PlayerChooseWeapon),
            despawn_choose_weapon_ui,
        );
        app.add_systems(
            Update,
            update_weapon_button_interaction.run_if(in_state(GameState::PlayerChooseWeapon)),
        );
    }
}

const HOVERED_BUTTON: Color = Color::rgb(0.80, 0.80, 0.80);
const NORMAL_BUTTON: Color = Color::rgb(1., 1., 1.);
const POPUP_BG_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.95);

fn despawn_choose_weapon_ui(mut commands: Commands, ui: Query<Entity, With<LevelUpUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}

fn spawn_choose_weapon_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
    camera: Query<&Camera>,
    player_weapon: Res<PlayerWeapons>,
) {
    let weapon_choose_parent = commands
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

    let weapon_choose_popup = commands
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
        .entity(weapon_choose_parent)
        .push_children(&[weapon_choose_popup]);

    let camera = camera.single();
    let view_dimensions = camera.logical_viewport_size().unwrap();
    let ratio = view_dimensions.x / SCREEN_WIDTH as f32;

    let player_weapons = player_weapon.weapons.clone();

    // max amount of time we try to find an upgrade before giving up
    let mut item_to_offer = 3;
    while item_to_offer > 0 {
        let mut potential_weapons = WeaponsTypes::list().clone();
        for player_weapon in &player_weapons {
            potential_weapons.retain(|&x| x != *player_weapon);
        }

        let Some(upgrade) = potential_weapons.choose(&mut rand::thread_rng()) else {
            // no more weapon available, go back to gameplay
            next_state.set(GameState::Gameplay);
            return;
        };

        // item is found, we can decrease the counter
        item_to_offer -= 1;

        let item_name = upgrade.name();

        let item_description = upgrade.name();

        let card_item = card_ui_factory(
            &mut commands,
            &asset_server,
            ratio,
            upgrade,
            &*item_name,
            &*item_description,
        );

        commands
            .entity(weapon_choose_popup)
            .push_children(&[card_item]);
    }
}

fn update_weapon_button_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonWeaponChoose), // UiImage
        (Changed<Interaction>, With<Button>),
    >,
    mut item_pickup: EventWriter<OnWeaponPickup>,
) {
    for (interaction, mut image, upgrade) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                item_pickup.send(OnWeaponPickup {
                    upgrade: upgrade.item,
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
    ratio: f32,
    upgrade: &WeaponsTypes,
    item_name: &str,
    item_description: &str,
) -> Entity {
    let texture = asset_server.load("item_ui_background_weapon.png");

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
            ButtonWeaponChoose { item: *upgrade },
        ))
        .id();

    let item_name = commands
        .spawn(
            TextBundle::from_section(
                item_name,
                TextStyle {
                    font: asset_server.load(FONT_BOLD),
                    font_size: 22.0,
                    color: Color::WHITE,
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
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(90.0),
                left: Val::Percent(7.0),
                width: Val::Percent(90.0),
                ..default()
            }),
        )
        .id();

    commands.entity(card_item).push_children(&[item_name]);
    commands
        .entity(card_item)
        .push_children(&[item_description]);

    card_item
}
