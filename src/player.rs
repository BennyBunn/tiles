use std::default;

use crate::input::{EventInput, InputType};
use bevy::math::vec3;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;
#[derive(Event)]
pub struct EventPlayerMoved(pub (f32, f32));

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, move_player)
            .add_event::<EventPlayerMoved>();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Artun.png");
    let layout: TextureAtlasLayout =
        TextureAtlasLayout::from_grid(UVec2::new(16, 16), 16, 16, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            texture: texture.clone(),
            atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(1.0 ))
                .with_translation(vec3(32.0, 32.0, 10.0)),
            ..default()
        },
        Player,
    ));
}

fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut update_player_pos: EventWriter<EventPlayerMoved>,
    mut input: EventReader<EventInput>,
    time: Res<Time>,
) {
    let mut player_transform = player_query.get_single_mut().unwrap();
    for ev in input.read() {
        match ev.0 {
            InputType::Left => player_transform.translation.x -= 10.0 * time.delta().as_secs_f32(),
            InputType::Right => player_transform.translation.x += 10.0 * time.delta().as_secs_f32(),
            InputType::Up => player_transform.translation.y += 10.0 * time.delta().as_secs_f32(),
            InputType::Down => player_transform.translation.y -= 10.0 * time.delta().as_secs_f32(),
            _ => (),
        }
    }
    update_player_pos.send(EventPlayerMoved((
        player_transform.translation.x,
        player_transform.translation.y,
    )));
}
