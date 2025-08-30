use bevy::{prelude::*, window::PrimaryWindow};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();
    app.register_type::<ScreenWrap>();
    app.add_systems(Update, (apply_movement, apply_screen_wrap));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// The direction the character wants to move in.
    pub intent: Vec2,
    /// Maximum speed in world units per second.
    pub max_speed: f32,
    /// Current velocity in world units per second.
    pub velocity: Vec2,
    /// Whether the character is on the ground.
    pub on_ground: bool,
    /// Whether the character is sliding.
    pub is_sliding: bool,
    /// Whether the character is wall sliding.
    pub is_wall_sliding: bool,
    /// Player requested a jump.
    pub want_jump: bool,
    /// Player requested to run.
    pub want_run: bool,
    /// Whether the character is running.
    pub is_running: bool,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            max_speed: 400.0,
            velocity: Vec2::ZERO,
            on_ground: false,
            is_sliding: false,
            is_wall_sliding: false,
            want_jump: false,
            want_run: false,
            is_running: false,
        }
    }
}

const GRAVITY: f32 = -1200.0; // world units / s^2 (tune for your pixel scale)
const JUMP_IMPULSE: f32 = 520.0; // vertical velocity applied when jumping

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&mut MovementController, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (mut controller, mut transform) in &mut movement_query {
        // horizontal velocity from intent
        controller.velocity.x = controller.intent.x * controller.max_speed;

        // gravity
        controller.velocity.y += GRAVITY * dt;

        // run
        if controller.want_run {
            controller.velocity.x *= 1.5;
        }

        // consider "running" only if run requested AND horizontal input is significant
        controller.is_running = controller.want_run && controller.intent.x.abs() > 0.5;
        info!("Running: {}", controller.is_running);


        // jump
        if controller.want_jump && controller.on_ground {
            controller.velocity.y = JUMP_IMPULSE;
            controller.on_ground = false;
        }
        // consume the jump request (single-frame)
        controller.want_jump = false;

        // integrate
        transform.translation += controller.velocity.extend(0.0) * dt;

        // naive ground detection: y <= 0.0 is ground. Replace with proper collision checks later.
        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            controller.on_ground = true;
            controller.velocity.y = 0.0;
        } else {
            controller.on_ground = false;
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScreenWrap;

fn apply_screen_wrap(
    window: Single<&Window, With<PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform, With<ScreenWrap>>,
) {
    let size = window.size() + 256.0;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);

    }
}
