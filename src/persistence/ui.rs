use crate::prelude::*;

#[derive(Component)]
pub struct ShareArmyButton;

#[derive(Component)]
pub struct LoadArmyButton;

#[cfg(target_arch = "wasm32")]
pub fn add_save_button(parent: &mut ChildBuilder) {
    // Evoke darkness.
    parent
        .spawn((
            ButtonBundle::default(),
            ShareArmyButton,
            Class::new("share_army"),
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "Share Army with Friends".to_string(),
                    TextStyle {
                        font: Default::default(),
                        font_size: 32.,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            },));
        });
}
#[cfg(not(target_arch = "wasm32"))]
pub fn add_save_button(parent: &mut ChildBuilder) {
    // Evoke darkness.
    parent
        .spawn((
            ButtonBundle::default(),
            ShareArmyButton,
            Class::new("share_army"),
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "Copy Army to Clipboard".to_string(),
                    TextStyle {
                        font: Default::default(),
                        font_size: 32.,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            },));
        });
}

pub fn add_load_button(parent: &mut ChildBuilder) {
    // Evoke darkness.
    parent
        .spawn((
            ButtonBundle::default(),
            ShareArmyButton,
            Class::new("summoner_battle"),
            LoadArmyButton,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "Battle Another Summoner".to_string(),
                    TextStyle {
                        font: Default::default(),
                        font_size: 32.,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            },));
        });
}

pub fn save_on_click(
    save_data: Option<Res<SaveData>>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<ShareArmyButton>)>,
) {
    for interaction in interactions.iter() {
        if *interaction == Interaction::Pressed && save_data.is_some() {
            let data = save_data.as_ref().unwrap();
            store_in_runes(&data.armies, true);
        }
    }
}

pub fn load_on_click(
    interactions: Query<&Interaction, (Changed<Interaction>, With<LoadArmyButton>)>,
    mut wave_assets: ResMut<Assets<SummonedMinions>>,
    mut summon_assets: ResMut<SummonsAssets>,
    mut story: ResMut<Story>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interactions.iter() {
        if *interaction == Interaction::Pressed {
            match retrieve_from_runes::<SaveData>() {
                Ok(save) => {
                    *story = Story::from_save_data(&save, &mut wave_assets, &mut summon_assets);
                    next_state.set(GameState::Looting);
                }
                Err(e) => {
                    info!("Error loading save: {:?}", e);
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
lazy_static! {
    static ref LOAD_STRING: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

#[cfg(target_arch = "wasm32")]
fn wait_for_loads() {
    use wasm_bindgen::prelude::*;
    let closure = Closure::new(move |s: String| {
        *LOAD_STRING.lock().unwrap() = Some(s.to_string());
        ()
    });
    set_loader(&closure);
    closure.forget();
}

#[cfg(target_arch = "wasm32")]
fn load_on_event(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    if let Some(save) = LOAD_STRING.lock().unwrap().take() {
        let save: SaveData = serde_json::from_str(&save).unwrap();
        commands.insert_resource(save.players.clone());
        commands.insert_resource(save.player_id.clone());
        commands.insert_resource(save.ai_seeds.clone());
        commands.insert_resource(save.map_desc);
        commands.insert_resource(save.turn_report);
        if let Some(evokation) = save
            .last_evokation
            .and_then(|evokation| read_from_runes::<Evokation>(&evokation, false))
        {
            if evokation.season == *save.season {
                commands.insert_resource(EvokingState::resume(evokation.clone(), &save.players));
                commands.insert_resource(evokation.player_turn);
            } else {
                commands.insert_resource(EvokingState::None {
                    last_evokation: Some(evokation),
                });
                commands.insert_resource(PlayerTurn::new(save.player_id));
            }
        } else {
            commands.insert_resource(PlayerTurn::new(save.player_id));
        }
        commands.insert_resource(save.season);
        next_state.set(GameState::Playing);
        hide_load();
    }
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct SaveData {
    pub armies: Vec<SummonedMinions>,
}
