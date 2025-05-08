use std::sync::{LazyLock, Mutex};

use bevy::{prelude::*, window::CursorGrabMode};
use web_sys::wasm_bindgen::{JsCast as _, prelude::Closure};

pub(super) fn plugin(app: &mut App) {
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

fn update_cursor_grab_status(mut windows: Query<&mut Window>) {
    let Some(locked) = POINTER_LOCKED.lock().unwrap().take() else {
        return;
    };
    for mut window in &mut windows {
        window.cursor_options.grab_mode = if locked {
            CursorGrabMode::Locked
        } else {
            CursorGrabMode::None
        };
    }
}
