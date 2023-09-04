use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{config, GameState};

#[derive(Component)]
pub struct Clickable;
#[derive(Component)]
pub struct Draggable;
#[derive(Component)]
pub struct LongClickable;

enum ClickPoint {
    Left,
    Right,
}

#[derive(Event)]
pub struct ClickPointEvent(ClickPoint);

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickPointEvent>()
            .add_systems(PreUpdate, click_with_story);
    }
}

// #[cfg(target_os = "wasm32")]
fn click_with_story(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<Input<MouseButton>>,
    app_state: Res<State<GameState>>,
    mut click_point: EventWriter<ClickPointEvent>,
) {
    match app_state.get() {
        GameState::Story => {
            if mouse.just_released(MouseButton::Left) {
                if let Some(position) = windows.single().cursor_position() {
                    if position.x < config::WINDOW_WIDTH / 2. {
                        println!("left");
                        println!("{:?}", position);
                        click_point.send(ClickPointEvent(ClickPoint::Left));
                    } else {
                        println!("right");
                        println!("{:?}", position);
                        click_point.send(ClickPointEvent(ClickPoint::Right));
                    }
                }
            }
        }
        // TODO: 다른 상태일 때 구현
        _ => return,
    }
}

#[cfg(target_os = "android")]
fn click() {}
