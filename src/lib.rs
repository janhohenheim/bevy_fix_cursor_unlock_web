#![warn(missing_docs)]
#![allow(clippy::type_complexity)]
#![doc = include_str!("../readme.md")]

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
#[cfg(target_arch = "wasm32")]
mod web;

/// Everything you need to fix the cursor unlock on web.
pub mod prelude {
    pub use super::{FixPointerUnlockPlugin, ForceUnlockCursor};
}

/// A tiny plugin that updates all [`Window`]s' [`CursorOptions::grab_mode`](bevy::window::CursorOptions) when the pointer
/// is unlocked on Web. This fixes <https://github.com/bevyengine/bevy/issues/8949>.
/// Does nothing on other platforms.
pub struct FixPointerUnlockPlugin;

impl Plugin for FixPointerUnlockPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        app.add_plugins(web::plugin);
        let _ = app;
    }
}

/// Event triggered when the cursor is forced to unlock, e.g. by pressing the Escape key when in fullscreen mode.
#[derive(EntityEvent, Copy, Clone, Debug)]
pub struct ForceUnlockCursor {
    /// The entity of the window that was unlocked.
    pub entity: Entity,
}
