//! Player-specific behavior.

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
    let layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 22, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();
    (
        Name::new("Player"),
        Player,
        Sprite {
            image: player_assets.idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animation.get_atlas_index(),
            }),
            ..default()
        },
        Transform::from_scale(Vec2::splat(2.0).extend(1.0)),
        MovementController {
            max_speed: 400.0,
            ..default()
        },
        ScreenWrap,
        player_animation,
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

/// Holds handles to player sprite assets.
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub idle: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            idle: assets.load("images/entities/player/player_idle.png"),
        }
    }
}
