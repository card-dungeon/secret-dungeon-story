use bevy::prelude::*;

#[derive(Component)]
pub struct Clickable;
#[derive(Component)]
pub struct Draggable;
#[derive(Component)]
pub struct LongClickable;

pub struct Controller;

impl Plugin for Controller {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, click);
    }
}

#[cfg(not(target_os = "android"))]
fn click() {}

#[cfg(target_os = "android")]
fn click() {}
