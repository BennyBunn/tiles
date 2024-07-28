use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use tiles::camera::CameraPlug;
use tiles::input::PlayerInputPlugin;
use tiles::player::PlayerPlugin;
// main c
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1200.0, 640.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, map)
        .add_plugins(CameraPlug)
        .add_plugins(PlayerPlugin)
        .add_plugins(PlayerInputPlugin)
        .insert_resource(LevelSelection::index(0))
        .run();
}



fn map(mut commands: Commands, asset_server: Res<AssetServer>) {
   
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk_maps.ldtk"),
        ..Default::default()
    });
}