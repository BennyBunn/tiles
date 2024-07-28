use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::player::EventPlayerMoved;

pub struct CameraPlug;

impl Plugin for CameraPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, camera_follow)
            .add_plugins(PanCamPlugin);
    }
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 1.0;
    // camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(16.0);
    // camera_bundle.projection.scale = 16.0;
    //commands.entity(camera_bundle.camera_2d.en);
    commands.spawn((
        camera_bundle,
        PanCam {
            grab_buttons: vec![MouseButton::Left, MouseButton::Middle], // which buttons should drag the camera
            enabled: true, // when false, controls are disabled. See toggle example.
            zoom_to_cursor: false, // whether to zoom towards the mouse or the center of the screen
            min_scale: 0.25, // prevent the camera from zooming too far in
            max_scale: Some(20.), // prevent the camera from zooming too far out
            ..default()
        },
    ));
}

fn camera_follow(
    mut cam_query: Query<&mut Transform, With<PanCam>>,
    mut event_player_moved: EventReader<EventPlayerMoved>,
) {
    let mut cam_transform = cam_query.get_single_mut().unwrap();
    for ev in event_player_moved.read() {
        let (x, y) = ev.0;
        cam_transform.translation.x = x;
        cam_transform.translation.y = y;
    }
}
