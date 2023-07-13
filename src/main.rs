use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};
use dotenv::dotenv;

mod config;
mod main_story_text;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Story,
    Select,
    Battle,
}

fn main() {
    dotenv().ok();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: config::GAME_TITLE.to_string(),
                        resizable: false,
                        resolution: (config::WINDOW_WIDTH, config::WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .init_resource::<main_story_text::GlobalStoryText>()
        .add_state::<GameState>()
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup)
        // .add_system(story_text_system)
        // .add_system(print_text_system)
        .add_system(show_story_text.in_set(OnUpdate(GameState::Story)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(360., 540., 100.)),
        ..default()
    });

    let font = asset_server.load("fonts/DungGeunMo.TTf");
    let text_style: TextStyle = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let box_size = Vec2::new(300.0, 200.0);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(
                    config::WINDOW_WIDTH / 1.2,
                    config::WINDOW_HEIGHT / 1.3,
                )),
                ..default()
            },
            transform: Transform::from_translation(Vec2::new(360.0, 540.0).extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "this text wraps in the boxasca ksucashci uas cuibasck1",
                            text_style.clone(),
                        )],
                        alignment: TextAlignment::Left,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
                    },
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(config::WINDOW_WIDTH / 1.3, config::WINDOW_HEIGHT / 1.4),
                    },
                    transform: Transform::from_translation(Vec3::Z),
                    ..default()
                },
                StoryText {
                    timer: Timer::from_seconds(config::TEXT_SPEED, TimerMode::Repeating),
                    story_pointer: 0,
                },
            ));
        });
}

#[derive(Component)]
struct StoryText {
    timer: Timer,
    story_pointer: i64,
}

fn show_story_text(
    story: Res<main_story_text::GlobalStoryText>,
    buttons: Res<Input<MouseButton>>,
    game_state: Res<State<GameState>>,
    mut query: Query<(&mut Text, With<StoryText>)>,
) {
    for mut text in &mut query {
        if buttons.just_pressed(MouseButton::Left) {
            println!("as")
        }
        // text.timer.tick(time.delta());

        // if text.timer.just_finished() {
        //     println!("Hello");
        // }
    }
}

// if buttons.pressed(MouseButton::Left) {
//     let s = story.0.clone();
//     println!("{:?}", s);
// }
