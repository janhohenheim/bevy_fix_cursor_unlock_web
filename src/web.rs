use std::sync::{LazyLock, Mutex};

use bevy::{
    input::{
        ButtonState, InputSystems,
        keyboard::{Key, KeyboardInput, keyboard_input_system},
    },
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use web_sys::wasm_bindgen::{JsCast as _, prelude::Closure};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WasUnlockedFromWebInTheLastFrame>();
    app.add_systems(Startup, setup_pointer_lock_change_event_listener);
    app.add_systems(
        PreUpdate,
        (release_virtual_escape_key, update_cursor_grab_status)
            .chain()
            .before(keyboard_input_system)
            .in_set(InputSystems),
    );
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
    mut cursor_options: Query<(Entity, &mut CursorOptions)>,
    mut keyboard_messages: MessageWriter<KeyboardInput>,
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
                keyboard_messages.write(keyboard_input(entity, ButtonState::Pressed));
                was_unlocked.0 = true;
            }
        }
    }
}

fn release_virtual_escape_key(
    mut was_unlocked: ResMut<WasUnlockedFromWebInTheLastFrame>,
    mut keyboard_messages: MessageWriter<KeyboardInput>,
    windows: Query<Entity, With<Window>>,
) {
    if !was_unlocked.0 {
        return;
    }
    was_unlocked.0 = false;
    for entity in &windows {
        keyboard_messages.write(keyboard_input(entity, ButtonState::Released));
    }
}

fn keyboard_input(window: Entity, state: ButtonState) -> KeyboardInput {
    KeyboardInput {
        key_code: KeyCode::Escape,
        logical_key: Key::Escape,
        state,
        text: None,
        repeat: false,
        window,
    }
}
