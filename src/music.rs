use std::time::Duration;

use crate::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(play_menu_music))
            .add_system_set(
                SystemSet::on_enter(GameState::Main).with_system(play_background_music),
            );
    }
}

pub fn play_menu_music(assets: Res<AssetServer>, audio: Res<bevy_kira_audio::prelude::Audio>) {
    let tween = AudioTween::new(Duration::from_millis(400), AudioEasing::Linear);
    audio.stop().fade_out(tween);
    audio.play(assets.load("Music/Title.wav")).looped();
}
pub fn play_background_music(
    assets: Res<AssetServer>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
) {
    let tween = AudioTween::new(Duration::from_millis(400), AudioEasing::Linear);
    audio.stop().fade_out(tween.clone());
    audio
        .play(assets.load("Music/Swamp.wav"))
        .looped()
        .fade_in(tween);
}
