use crate::{KeyEvent, KeyboardRegister, StartProgramEvent};
use bevy::prelude::*;
use bevy_midi_graph::{
    config::{Config, SoundSource},
    MidiGraphAudioContext,
};

const PROGRAM_NO: usize = 0;
const NODE_ID_LOWER: u64 = 0;
const NODE_ID_UPPER: u64 = 1;

pub struct OutputPlugin;

impl Plugin for OutputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_audio)
            .add_systems(Update, play_key_events)
            .add_systems(PostUpdate, change_program);
    }
}

fn configure_audio(mut audio_context: ResMut<MidiGraphAudioContext>) {
    let config = Config {
        root: SoundSource::stock_square_wave(),
    };
    audio_context
        .store_new_program(PROGRAM_NO, &config)
        .unwrap();
    audio_context.change_program(PROGRAM_NO).unwrap();
}

fn play_key_events(
    mut events: EventReader<KeyEvent>,
    mut audio_context: ResMut<MidiGraphAudioContext>,
) {
    if events.is_empty() {
        return;
    }
    for event in events.read() {
        let node_id = match event.register {
            KeyboardRegister::Lower => NODE_ID_LOWER,
            KeyboardRegister::Upper => NODE_ID_UPPER,
        };
        let event_channel = audio_context
            .event_channel(node_id)
            .unwrap()
            .expect("No event channel found");
        event_channel
            .send(event.event.clone())
            .expect("INTERNAL: Send failure");
    }
}

fn change_program(
    mut events: EventReader<StartProgramEvent>,
    mut audio_context: ResMut<MidiGraphAudioContext>,
) {
    for event in events.read() {
        audio_context.change_program(event.program_no).unwrap();
        println!("DID CHANGE PROGRAM: {}", event.program_no);
    }
}
