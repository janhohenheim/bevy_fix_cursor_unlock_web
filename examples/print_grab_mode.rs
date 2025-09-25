use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_fix_cursor_unlock_web::FixPointerUnlockPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FixPointerUnlockPlugin)
        .add_systems(Startup, setup_text)
        .add_systems(Update, print_grab_mode)
        .add_systems(
            Update,
            lock_cursor.run_if(input_just_pressed(MouseButton::Left)),
        )
        .run();
}

fn setup_text(mut commands: Commands) {
    commands.spawn(Text::default());
    commands.spawn(Camera2d);
}

fn print_grab_mode(
    cursor_options: Single<&CursorOptions, Changed<CursorOptions>>,
    mut text: Single<&mut Text>,
) {
    let grab_mode = cursor_options.grab_mode;
    // Without this plugin, this would report `CursorGrabMode::Locked` even when the cursor is unlocked on Web.
    text.0 = format!("grab_mode: {:?}", grab_mode);
}

fn lock_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
}
