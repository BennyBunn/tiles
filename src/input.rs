use bevy::prelude::*;

pub struct PlayerInputPlugin;

#[derive(Event)]
pub struct EventInput(pub InputType);

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, key_pressed)
            .add_event::<EventInput>();
    }
}

pub enum InputType {
    Up,
    Down,
    Right,
    Left,
}

fn key_pressed(mut update_input: EventWriter<EventInput>, input: Res<ButtonInput<KeyCode>>) {
    let mut input_type: Option<InputType> = None;

    if input.pressed(KeyCode::KeyA) {
        input_type = Some(InputType::Left);
    }
    if input.pressed(KeyCode::KeyD) {
        input_type = Some(InputType::Right);
    }
    if input.pressed(KeyCode::KeyW) {
        input_type = Some(InputType::Up);
    }
    if input.pressed(KeyCode::KeyS) {
        input_type = Some(InputType::Down);
    }

    if let Some(v) = input_type {
        update_input.send(EventInput(v));
    }
}
