use crate::StartProgramEvent;
use bevy::{asset::LoadState, prelude::*};
use bevy_midi_graph::{MidiGraphAsset, MidiGraphAudioContext};

const DEFAULT_PROGRAM: usize = 1;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProgramAssets>()
            .add_systems(Startup, init_program_assets)
            .add_systems(Update, check_graph_assets_ready);
    }
}

#[derive(Resource, Default)]
pub struct ProgramAssets {
    pub programs: Vec<(LoadState, usize, Handle<MidiGraphAsset>)>,
}

fn init_program_assets(server: Res<AssetServer>, mut program_data: ResMut<ProgramAssets>) {
    let asset_names = [
        "f1.ron", "f2.ron", "f3.ron", "f4.ron", "f5.ron", "f6.ron", "f7.ron", "f8.ron", "f9.ron",
        "f10.ron", "f11.ron", "f12.ron",
    ];
    for (program_no, name) in asset_names.iter().enumerate() {
        program_data
            .programs
            .push((LoadState::NotLoaded, program_no, server.load(*name)));
    }
}

fn check_graph_assets_ready(
    server: Res<AssetServer>,
    graph_assets: Res<Assets<MidiGraphAsset>>,
    mut program_data: ResMut<ProgramAssets>,
    mut audio_context: ResMut<MidiGraphAudioContext>,
    mut events: EventWriter<StartProgramEvent>,
    mut completed: Local<bool>,
) {
    if *completed {
        return;
    }
    for asset in program_data.programs.iter_mut() {
        match asset.0 {
            LoadState::Loaded | LoadState::Failed(_) => continue,
            _ => {}
        };
        let updated_load_state = server.load_state(asset.2.id());
        match updated_load_state {
            LoadState::Failed(_) | LoadState::NotLoaded => {
                asset.0 = updated_load_state;
                continue;
            }
            LoadState::Loaded | LoadState::Loading => {
                let is_loaded = server.is_loaded_with_dependencies(asset.2.id());
                if !is_loaded {
                    return;
                }
                asset.0 = LoadState::Loaded;
                let graph = graph_assets.get(&asset.2).unwrap();
                audio_context
                    .store_new_program(asset.1, &graph.config)
                    .unwrap();
                println!("DID STORE PROGRAM: {}", asset.1);
            }
        }
    }

    *completed = true;
    events.send(StartProgramEvent {
        program_no: DEFAULT_PROGRAM,
    });
}