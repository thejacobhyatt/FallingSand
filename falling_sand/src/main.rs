use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // run these only once at launch
        .add_systems(Startup, setup)
        // run these every frame update
        .add_systems(Update, cursor_position)
        // ...
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn the 2D camera
    commands.spawn(Camera2dBundle::default());
}

fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}
