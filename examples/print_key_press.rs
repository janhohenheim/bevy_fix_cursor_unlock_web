use bevy::{
    input::{common_conditions::input_just_pressed, keyboard::KeyboardInput},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_fix_cursor_unlock_web::FixPointerUnlockPlugin;

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
        .run();
}

fn setup_text(mut commands: Commands) {
    commands.spawn(Text::new("Please press the Escape key"));
    commands.spawn(Camera2d);
}

fn print_key_press(mut text: Single<&mut Text>, mut keyboard_input: MessageReader<KeyboardInput>) {
    for message in keyboard_input.read() {
        // Without this plugin, this would not report the `Esc` key press used to unlock the cursor on Web.
        text.0 = format!("Keyboard input message: {message:#?}");
    }
}

fn lock_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
}
