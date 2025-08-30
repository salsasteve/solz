use std::collections::HashMap;

use bevy::prelude::*;

use crate::demo::animation::{PlayerAnimation, PlayerAnimationState};
use crate::demo::movement::{MovementController, ScreenWrap};

/// Registers the player component and input system.
pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.register_type::<PlayerAssets>();
    app.init_resource::<PlayerAssets>();
    app.add_systems(Startup, init_player_skin);
    app.add_systems(Update, record_player_directional_input);
}

/// Returns a bundle of components for spawning the player entity.
pub fn setup_player(
    player_skin: Res<PlayerSkin>
) -> impl Bundle {
    // let idle_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 22, 1, None, None);
    // let idle_handle = texture_atlas_layouts.add(idle_layout);
    // let running_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 8, 1, None, None);
    // let running_handle = texture_atlas_layouts.add(running_layout);
    // let jumping_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 1, 1, None, None);
    // let jumping_handle = texture_atlas_layouts.add(jumping_layout);
    // let sliding_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 1, 1, None, None);
    // let sliding_handle = texture_atlas_layouts.add(sliding_layout);
    // let wall_sliding_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 1, 1, None, None);
    // let wall_sliding_handle = texture_atlas_layouts.add(wall_sliding_layout);
    let player_animation = PlayerAnimation::new();
    let idle_visual = player_skin.get(PlayerAnimationState::Idling).clone();

    let sprite = Sprite {
        image: idle_visual.image.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: idle_visual.atlas.unwrap().clone(),
            index: player_animation.get_atlas_index(),
        }),
        ..default()
    };

    (
        Name::new("Player"),
        Player,
        sprite,
        Transform::from_scale(Vec2::splat(2.5).extend(1.0)),
        MovementController::default(),
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
    const WALK_SPEED: f32 = 100.0;
    const RUN_SPEED: f32 = 300.0;

    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }
    let intent = intent.normalize_or_zero();

    let jump_pressed = input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::ArrowUp);
    let run_pressed = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);

    for mut controller in &mut controller_query {
        controller.intent = intent;
        if jump_pressed {
            controller.want_jump = true;
            info!("Jump pressed");
        }

        // If holding shift and moving horizontally, use run speed; otherwise walk speed.
        if run_pressed && intent.x.abs() > 0.0 {
            controller.max_speed = RUN_SPEED;
            controller.want_run = true;
        } else {
            controller.max_speed = WALK_SPEED;
        }
    }
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

#[derive(Clone)]
pub struct PlayerVisual {
    pub image: Handle<Image>,
    /// None = single-frame image only. Some(layout) = atlas-based sprite sheet.
    pub atlas: Option<Handle<TextureAtlasLayout>>,
    /// number of frames in the atlas (optional, for get_atlas_index)
    pub frames: usize,
}

#[derive(Resource, Clone)]
pub struct PlayerSkin {
    pub visuals: HashMap<PlayerAnimationState, PlayerVisual>,
}

impl PlayerSkin {
    pub fn get(&self, state: PlayerAnimationState) -> PlayerVisual {
        if let Some(v) = self.visuals.get(&state) {
            return v.clone();
        }
        // fallback to idling if available, otherwise use any visual
        warn!("PlayerSkin missing visual for {:?}; falling back to Idling/first available", state);
        if let Some(v) = self.visuals.get(&PlayerAnimationState::Idling) {
            return v.clone();
        }
        self.visuals
            .values()
            .next()
            .expect("PlayerSkin has no visuals configured")
            .clone()
    }
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

// create atlas layouts and insert the PlayerSkin resource
fn init_player_skin(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let idle_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 22, 1, None, None);
    let idle_atlas = texture_atlas_layouts.add(idle_layout);

    let running_layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 8, 1, None, None);
    let running_atlas = texture_atlas_layouts.add(running_layout);

    let jumping_layout = TextureAtlasLayout::from_grid(UVec2::new(1, 1), 1, 1, None, None);
    let jumping_atlas = texture_atlas_layouts.add(jumping_layout);

    // build visuals map
    let mut visuals = HashMap::new();
    visuals.insert(
        PlayerAnimationState::Idling,
        PlayerVisual { image: player_assets.idle.clone(), atlas: Some(idle_atlas.clone()), frames: 22 },
    );
    visuals.insert(
        PlayerAnimationState::Walking,
        PlayerVisual { image: player_assets.idle.clone(), atlas: Some(idle_atlas.clone()), frames: 22 },
    );
    visuals.insert(
        PlayerAnimationState::Running,
        PlayerVisual { image: player_assets.running.clone(), atlas: Some(running_atlas.clone()), frames: 8 },
    );
    visuals.insert(
        PlayerAnimationState::Jumping,
        PlayerVisual { image: player_assets.jumping.clone(), atlas: None, frames: 1 },
    );
    // add sliding, wall_sliding, etc.

    commands.insert_resource(PlayerSkin { visuals });
}
