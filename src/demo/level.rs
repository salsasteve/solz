use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    demo::{helpers, player::{self, setup_player}},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_skin: Res<player::PlayerSkin>,
    asset_server: Res<AssetServer>,
) {
    info!("Spawning level and music");

    // Check if music asset is loaded
    info!("Music handle: {:?}", level_assets.music);
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            setup_player(player_skin),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            )
        ],
    ));
    let map_handle = helpers::tiled::TiledMapHandle(asset_server.load("maps/map1.tmx"));

    commands.spawn(helpers::tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}
