use bevy::window::PresentMode;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use prelude::*;

pub const HEIGHT: f32 = 900.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

mod enemy;
mod input;
mod mouse;
mod player;
mod prelude;

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(texture_atlas(
        tile_size_x = 1.,
        tile_size_y = 1.,
        columns = 1,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "white_pixel.png")]
    player: Handle<TextureAtlas>,
    #[asset(texture_atlas(
        tile_size_x = 512.,
        tile_size_y = 512.,
        columns = 1,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "awesome.png")]
    enemy: Handle<TextureAtlas>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("b0c060").unwrap()))
        .add_state(GameState::Splash)
        .add_loading_state(
            LoadingState::new(GameState::Splash)
                .continue_to_state(GameState::Main)
                .with_collection::<GameAssets>(),
        )
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Potion Game".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PhysicsPlugin::default())
        //Our Plugins
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        //One off weird systems
        .add_startup_system(spawn_camera)
        .insert_resource(MousePos::default())
        .add_system(mouse_position)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
