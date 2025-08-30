//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod core;
mod entities;
mod gameplay;
mod story;
mod animation;
mod helpers;
pub mod level;
mod movement;
mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        core::CorePlugin,
        entities::EntityPlugins,
        gameplay::GameplayPlugin,
        story::StoryPlugin,
    ));

    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
    ));
    app.add_plugins(TilemapPlugin);
    app.add_plugins(helpers::tiled::TiledMapPlugin);
}


// use bevy::prelude::*;
// use bevy_ecs_tilemap::prelude::*;

// mod core;
// mod entities;
// mod gameplay;
// mod story;
// pub mod helpers;
// pub mod level;

// pub struct DemoPlugin;

// impl Plugin for DemoPlugin {
//     fn build(&self, app: &mut App) {
//         // Add new internal plugins (empty for now)
//         app.add_plugins((
//             core::CorePlugin,
//             entities::EntityPlugins,
//             gameplay::GameplayPlugin,
//             story::StoryPlugin,
//         ));

//         // Keep ALL existing systems working
//         app.add_plugins((
//             animation::plugin,
//             level::plugin,
//             movement::plugin,
//             player::plugin,
//         ));
//         app.add_plugins(TilemapPlugin);
//         app.add_plugins(helpers::tiled::TiledMapPlugin);
//     }
// }
