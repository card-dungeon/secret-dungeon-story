use bevy::prelude::*;

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

pub struct Controller;

impl Plugin for Controller {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, click);
    }
}

#[cfg(target_os = "wasm32")]
fn click(windows: Res<Windows>, mut click_point: EventWriter<ClickPointEvent>) {
    if mouse.just_released(MouseButton::Left) {
        if let Some() = mouse.position() {
            click_point.send(ClickPointEvent(ClickPoint::Left));
        }
        click_point.send(ClickPointEvent(ClickPoint::Left));
    }
}

#[cfg(target_os = "android")]
fn click() {}
