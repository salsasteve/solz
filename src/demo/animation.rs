//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use bevy::prelude::*;
use std::time::Duration;

use crate::demo::{movement::MovementController, player::Player, player::PlayerSkin};

/// Registers the player animation component and systems.
pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAnimation>();
    app.add_systems(
        Update,
        (
            update_animation_timer,
            update_animation_movement,
            update_animation_atlas,
        ),
    );
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
    mut player_query: Query<(&MovementController, &mut Sprite, &mut PlayerAnimation)>,
) {
    // for (controller, mut sprite, mut animation) in &mut player_query {
    //     let dx = controller.intent.x;
    //     if dx != 0.0 {
    //         sprite.flip_x = dx < 0.0;
    //     }

    //     let animation_state = if controller.intent == Vec2::ZERO {
    //         PlayerAnimationState::Idling
    //     } else {
    //         PlayerAnimationState::Walking
    //     };
    //     animation.update_state(animation_state);
    // }
    for (controller, mut sprite, mut animation) in &mut player_query {
        // flip sprite based on horizontal intent
        let dx = controller.intent.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        // Decide animation state.
        // Try to use controller flags if available (rename fields below to match your MovementController).
        let state = if cfg!(any()) {
            // placeholder; real logic below
            PlayerAnimationState::Idling
        } else {
            if controller.is_wall_sliding {
                PlayerAnimationState::WallSliding
            } else if !controller.on_ground {
                PlayerAnimationState::Jumping
            } else if controller.is_sliding {
                PlayerAnimationState::Sliding
            } else if controller.is_running {
                PlayerAnimationState::Running
            } else if controller.intent != Vec2::ZERO {
                PlayerAnimationState::Walking
            } else {
                PlayerAnimationState::Idling
            }
        };

        animation.update_state(state);
    }
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(
    player_skin: Res<PlayerSkin>,
    mut query: Query<(&PlayerAnimation, &mut Sprite), With<Player>>,
) {
    for (animation, mut sprite) in &mut query {
        let visual = player_skin.get(animation.state.clone());

        // ensure the sprite uses the correct base image for this state
        sprite.image = visual.image.clone();

        match visual.atlas {
            Some(ref layout) => {
                // compute a safe atlas index (wrap if necessary)
                let mut idx = animation.get_atlas_index();
                if visual.frames > 0 {
                    idx = idx % visual.frames;
                }

                // ensure atlas exists and uses the correct layout + index
                if sprite.texture_atlas.is_none() {
                    sprite.texture_atlas = Some(TextureAtlas {
                        layout: layout.clone(),
                        index: idx,
                    });
                    continue;
                }

                if let Some(atlas) = sprite.texture_atlas.as_mut() {
                    if atlas.layout != *layout {
                        info!("Swapping atlas for state {:?} (index {})", animation.state, idx);
                        atlas.layout = layout.clone();
                        atlas.index = idx;
                    } else if animation.changed() {
                        atlas.index = idx;
                    }
                }
            }
            None => {
                // single-image visual: clear atlas so renderer uses sprite.image only
                sprite.texture_atlas = None;
            }
        }
    }
}


#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}

#[derive(Clone,Debug, Reflect, PartialEq, Eq, Hash)]
pub enum PlayerAnimationState {
    Idling,
    Walking,
    Running,
    Jumping,
    Sliding,
    WallSliding,
}

impl PlayerAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 22;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(50);
    /// The number of walking frames.
    const WALKING_FRAMES: usize = 22;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(50);
    /// The number of running frames.
    const RUNNING_FRAMES: usize = 8;
    /// The duration of each running frame.
    const RUNNING_INTERVAL: Duration = Duration::from_millis(50);
    /// The number of jumping frames.
    const JUMPING_FRAMES: usize = 1;
    /// The duration of each jumping frame.
    const JUMPING_INTERVAL: Duration = Duration::from_millis(50);
    /// The number of sliding frames.
    const SLIDING_FRAMES: usize = 1;
    /// The duration of each sliding frame.
    const SLIDING_INTERVAL: Duration = Duration::from_millis(50);
    /// The number of wall sliding frames.
    const WALL_SLIDING_FRAMES: usize = 1;
    /// The duration of each wall sliding frame.
    const WALL_SLIDING_INTERVAL: Duration = Duration::from_millis(50);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling,
        }
    }

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walking,
        }
    }

    fn running() -> Self {
        Self {
            timer: Timer::new(Self::RUNNING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Running,
        }
    }

    fn jumping() -> Self {
        Self {
            timer: Timer::new(Self::JUMPING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Jumping,
        }
    }

    fn sliding() -> Self {
        Self {
            timer: Timer::new(Self::SLIDING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Sliding,
        }
    }

    fn wall_sliding() -> Self {
        Self {
            timer: Timer::new(Self::WALL_SLIDING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::WallSliding,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                PlayerAnimationState::Idling => Self::IDLE_FRAMES,
                PlayerAnimationState::Walking => Self::WALKING_FRAMES,
                PlayerAnimationState::Running => Self::RUNNING_FRAMES,
                PlayerAnimationState::Jumping => Self::JUMPING_FRAMES,
                PlayerAnimationState::Sliding => Self::SLIDING_FRAMES,
                PlayerAnimationState::WallSliding => Self::WALL_SLIDING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idling => *self = Self::idling(),
                PlayerAnimationState::Walking => *self = Self::walking(),
                PlayerAnimationState::Running => *self = Self::running(),
                PlayerAnimationState::Jumping => *self = Self::jumping(),
                PlayerAnimationState::Sliding => *self = Self::sliding(),
                PlayerAnimationState::WallSliding => *self = Self::wall_sliding(),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            PlayerAnimationState::Idling => self.frame,
            PlayerAnimationState::Walking => self.frame,
            PlayerAnimationState::Running => self.frame,
            PlayerAnimationState::Jumping => self.frame,
            PlayerAnimationState::Sliding => self.frame,
            PlayerAnimationState::WallSliding => self.frame,
        }
    }
}
