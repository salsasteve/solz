use bevy::prelude::*;

use crate::demo::animation::PlayerAnimation;
use crate::demo::movement::{MovementController, ScreenWrap};

/// Registers the player component and input system.
pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.register_type::<PlayerAssets>();
    app.init_resource::<PlayerAssets>();
    app.add_systems(Update, record_player_directional_input);
}

/// Returns a bundle of components for spawning the player entity.
pub fn setup_player(
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> impl Bundle {
    let idle_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 22, 1, None, None);
    let idle_handle = texture_atlas_layouts.add(idle_layout);
    let running_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 8, 1, None, None);
    let running_handle = texture_atlas_layouts.add(running_layout);
    let jumping_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 1, 1, None, None);
    let jumping_handle = texture_atlas_layouts.add(jumping_layout);
    let sliding_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 1, 1, None, None);
    let sliding_handle = texture_atlas_layouts.add(sliding_layout);
    let wall_sliding_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 1, 1, None, None);
    let wall_sliding_handle = texture_atlas_layouts.add(wall_sliding_layout);
    let player_animation = PlayerAnimation::new();
    (
        Name::new("Player"),
        Player,
        Sprite {
            image: player_assets.idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: idle_handle.clone(),
                index: player_animation.get_atlas_index(),
            }),
            ..default()
        },
        Transform::from_scale(Vec2::splat(2.5).extend(1.0)),
        MovementController {
            max_speed: 100.0,
            ..default()
        },
        ScreenWrap,
        player_animation,
        PlayerAtlases {
            idle: idle_handle,
            running: running_handle,
            jumping: jumping_handle,
            sliding: sliding_handle,
            wall_sliding: wall_sliding_handle,
        },
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

/// System to record player movement input and update the controller intent.
fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }
    let intent = intent.normalize_or_zero();
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct PlayerAtlases {
    pub idle: Handle<TextureAtlasLayout>,
    pub running: Handle<TextureAtlasLayout>,
    pub jumping: Handle<TextureAtlasLayout>,
    pub sliding: Handle<TextureAtlasLayout>,
    pub wall_sliding: Handle<TextureAtlasLayout>,
}

/// Holds handles to player sprite assets.
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
    pub idle: Handle<Image>,
    pub jumping: Handle<Image>,
    pub running: Handle<Image>,
    pub sliding: Handle<Image>,
    pub wall_sliding: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
            idle: assets.load("images/entities/player/idle.png"),
            jumping: assets.load("images/entities/player/jumping.png"),
            running: assets.load("images/entities/player/running.png"),
            sliding: assets.load("images/entities/player/sliding.png"),
            wall_sliding: assets.load("images/entities/player/wall_sliding.png"),
        }
    }
}
