use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct ProgressStatus {
    pub main_story: String,
    pub main_story_progress: usize,
}
impl FromWorld for ProgressStatus {
    #[allow(unused_variables)]
    fn from_world(world: &mut World) -> Self {
        ProgressStatus {
            main_story: "myunghun_001".to_string(),
            main_story_progress: 0,
        }
    }
}
