use bevy::{
    input::{common_conditions::input_just_pressed, keyboard::KeyboardInput},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_fix_cursor_unlock_web::{FixPointerUnlockPlugin, ForceUnlockCursor};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FixPointerUnlockPlugin)
        .add_systems(Startup, setup_text)
        .add_systems(Update, print_key_press)
        .add_systems(
            Update,
            lock_cursor.run_if(input_just_pressed(MouseButton::Left)),
        )
        .add_observer(print_force_unlock)
        .run();
}

fn setup_text(mut commands: Commands) {
    commands.spawn(Text::new(
        "Click to lock cursor, press any key to debug print it",
    ));
    commands.spawn(Camera2d);
}

fn print_key_press(mut text: Single<&mut Text>, mut keyboard_input: MessageReader<KeyboardInput>) {
    for message in keyboard_input.read() {
        text.0 = format!("Keyboard input message: {message:#?}");
    }
}

fn print_force_unlock(_force_unlock: On<ForceUnlockCursor>, mut text: Single<&mut Text>) {
    text.0 = format!("No keyboard event, but cursor was forced to unlock");
}

fn lock_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
}
