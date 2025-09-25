use std::sync::{LazyLock, Mutex};

use super::ForceUnlockCursor;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_window::{CursorGrabMode, CursorOptions};
use web_sys::wasm_bindgen::{JsCast as _, prelude::Closure};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WasUnlockedFromWebInTheLastFrame>();
    app.add_systems(Startup, setup_pointer_lock_change_event_listener);
    app.add_systems(PreUpdate, update_cursor_grab_status);
}

static POINTER_LOCKED: LazyLock<Mutex<Option<bool>>> = LazyLock::new(|| Mutex::new(None));

fn setup_pointer_lock_change_event_listener() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let document_inner = document.clone();
    let cb: Closure<dyn Fn(web_sys::Event)> = Closure::new(move |_event: web_sys::Event| {
        let element = document_inner.pointer_lock_element();
        POINTER_LOCKED.lock().unwrap().replace(element.is_some());
    });
    document
        .add_event_listener_with_callback("pointerlockchange", cb.as_ref().unchecked_ref())
        .unwrap();

    cb.into_js_value();
}

#[derive(Resource, Default)]
struct WasUnlockedFromWebInTheLastFrame(bool);

fn update_cursor_grab_status(
    mut commands: Commands,
    mut cursor_options: Query<(Entity, &mut CursorOptions)>,
    mut was_unlocked: ResMut<WasUnlockedFromWebInTheLastFrame>,
) {
    let Some(locked) = POINTER_LOCKED.lock().unwrap().take() else {
        return;
    };
    for (entity, mut cursor_options) in &mut cursor_options {
        let new_state = if locked {
            CursorGrabMode::Locked
        } else {
            CursorGrabMode::None
        };
        if cursor_options.grab_mode != new_state {
            cursor_options.grab_mode = new_state;
            if new_state == CursorGrabMode::None {
                commands.trigger(ForceUnlockCursor { entity });
                was_unlocked.0 = true;
            }
        }
    }
}
