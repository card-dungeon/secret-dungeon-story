use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct ProgressStatus {
    pub story_part: String,
    pub current_part: usize,
    pub text_progress: usize,
}
