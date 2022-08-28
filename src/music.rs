use crate::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system(play_background_music);
    }
}

pub fn play_background_music(
    assets: Res<AssetServer>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
) {
    audio.play(assets.load("Music/Swamp.wav")).looped();
}
