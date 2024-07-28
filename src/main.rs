use bevy::prelude::*;

// main c
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1200.0, 640.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .run();
}
