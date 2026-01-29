# Fix Cursor Unlock on Web

[![crates.io](https://img.shields.io/crates/v/bevy_fix_cursor_unlock_web)](https://crates.io/crates/bevy_fix_cursor_unlock_web)
[![docs.rs](https://docs.rs/bevy_fix_cursor_unlock_web/badge.svg)](https://docs.rs/bevy_fix_cursor_unlock_web)

A tiny plugin that fixes Bevy not reporting when the cursor is unlocked on web


## Usage

Just add the plugin, that's it:

```rust,no_run
use bevy::prelude::*;
use bevy_fix_cursor_unlock_web::prelude::*;

App::new()
  .add_plugins(DefaultPlugins)
  .add_plugins(FixPointerUnlockPlugin);
```

Now, `Window::cursor_options::grab_mode` is automatically set to `CursorGrabMode::None` for you when unlocking the cursor on web.
This fixes <https://github.com/bevyengine/bevy/issues/8949>

If you want to detect when a cursor was force freed, e.g. when the user presses escape, you can observe the `ForceUnlockCursor` event.
Note that Bevy does *not* forward the "Escape" key press in this case.

## Compatibility

| bevy        | bevy_fix_cursor_unlock_web |
|-------------|------------------------|
| 0.18        | 0.3                    |
| 0.17        | 0.2                    |
| 0.16        | 0.1                    |
