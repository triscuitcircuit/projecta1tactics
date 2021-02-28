use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};
use bevy_inspector_egui::InspectorPlugin;
use std::collections::HashMap;
use bevy::prelude::*;
use bevy::asset::LoadState;

// Sample code from Kira_audio github

pub struct AudioState{
    audio_loaded: bool,
    loop_handle: Handle<AudioSource>,
    sound_handle: Handle<AudioSource>,
    channels: HashMap<AudioChannel, ChannelAudioState>
}
struct ChannelAudioState{
    stopped: bool,
    paused: bool,
    loop_started: bool,
    volume: f32,
}
impl Default for ChannelAudioState{
    fn default() -> Self {
        ChannelAudioState{
            stopped: true,
            paused: false,
            loop_started: false,
            volume: 1.0
        }
    }
}


pub fn start_loop(
    audio: Res<Audio>,
    mut audio_state: ResMut<AudioState>
) {
    if !audio_state.audio_loaded {
        return;
    }
    audio.play_looped(audio_state.loop_handle.clone());

    // for (interaction, button) in interaction_query.iter_mut() {
    //     let mut channel_audio_state = audio_state.channels.get_mut(&button.channel).unwrap();
    //     if channel_audio_state.loop_started {
    //         continue;
    //     }
    //     if interaction == &Interaction::Clicked {
    //         channel_audio_state.loop_started = true;
    //         channel_audio_state.stopped = false;
    //         audio.play_looped_in_channel(audio_state.loop_handle.clone(), &button.channel);
    //     }
    // }
}

pub fn check_audio_loading(mut audio_state: ResMut<AudioState>, asset_server: ResMut<AssetServer>){
    if audio_state.audio_loaded
        || LoadState::Loaded != asset_server.get_load_state(&audio_state.loop_handle)
        || LoadState::Loaded != asset_server.get_load_state(&audio_state.sound_handle)
    {
        return;
    }
    audio_state.audio_loaded = true;
}

pub fn prepare_audio(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>
){
    let mut channels = HashMap::new();
    channels.insert(
        AudioChannel::new("music".to_owned()),
        ChannelAudioState::default()
    );
    channels.insert(
        AudioChannel::new("impact".to_owned()),
        ChannelAudioState::default()
    );
    channels.insert(
        AudioChannel::new("movement".to_owned()),
        ChannelAudioState::default()
    );

    let loop_handle = asset_server.load("sounds/Battle1(Looped).wav");
    let sound_handle = asset_server.load("sounds/SFX_-_magic_spell_02.ogg");

    let audio_state = AudioState{
        audio_loaded: false,
        loop_handle,
        sound_handle,
        channels
    };
    commands.insert_resource(audio_state);
}
