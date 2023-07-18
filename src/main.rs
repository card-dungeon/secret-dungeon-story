use std::time::Duration;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};
use dotenv::dotenv;

mod character;
mod config;
mod main_story_text;
mod progress_status;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Story,
    Select,
    Battle,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

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
                    watch_for_changes: Some(bevy::asset::ChangeWatcher {
                        delay: Duration::from_secs(1),
                    }),
                    ..default()
                }),
        )
        .init_resource::<main_story_text::GlobalStoryText>()
        .init_resource::<progress_status::ProgressStatus>()
        .add_state::<GameState>()
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, next_text)
        .add_systems(Update, pre_text)
        .add_systems(Update, show_text)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(360., 540., 100.)),
        ..default()
    });

    let text_style: TextStyle = TextStyle {
        font: asset_server.load("fonts/DungGeunMo.TTf"),
        font_size: 60.0,
        color: Color::WHITE,
    };

    // 메인 스토리 텍스트
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
                            "this text wraps in the boxasca ksucashci uas cuibasck1asuicgh",
                            text_style.clone(),
                        )],
                        alignment: TextAlignment::Left,
                        linebreak_behavior: BreakLineOn::WordBoundary,
                    },
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(config::WINDOW_WIDTH / 1.3, config::WINDOW_HEIGHT / 1.4),
                    },
                    transform: Transform::from_translation(Vec3::Z),
                    ..default()
                },
                StoryText,
            ));
        });

    // 앞으로 가기 버튼
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(config::WINDOW_WIDTH / 3.),
                            height: Val::Px(config::WINDOW_HEIGHT / 10.),
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        transform: Transform::from_xyz(
                            config::WINDOW_WIDTH,
                            config::WINDOW_HEIGHT,
                            1.,
                        ),
                        ..default()
                    },
                    NextButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("->", text_style.clone()));
                });
        })
        // 뒤로 가기 버튼
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(config::WINDOW_WIDTH / 3.),
                            height: Val::Px(config::WINDOW_HEIGHT / 10.),
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        transform: Transform::from_translation(Vec3::new(
                            -config::WINDOW_WIDTH / 3.,
                            0.0,
                            0.0,
                        )),
                        ..default()
                    },
                    PreButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("<-", text_style.clone()));
                });
        });
}

#[derive(Component)]
struct StoryText;

#[derive(Component)]
struct NextButton;
#[derive(Component)]
struct PreButton;

fn next_text(
    story_text: Res<main_story_text::GlobalStoryText>,
    mut progress: ResMut<progress_status::ProgressStatus>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>, With<NextButton>),
    >,
    mut textbox: Query<(&mut Text, With<StoryText>)>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                // TODO: 삭제
                // if progress.main_story_progress == text.len() - 1 as usize {
                //     progress.main_story_progress = text.len() - 1;
                // } else {
                progress.main_story_progress += 1;
                // }
            }
            Interaction::Hovered => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn pre_text(
    mut progress: ResMut<progress_status::ProgressStatus>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>, With<PreButton>),
    >,
    mut textbox: Query<(&mut Text, With<StoryText>)>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                // TODO: 삭제
                if progress.main_story_progress == 0 {
                    progress.main_story_progress = 0;
                } else {
                    progress.main_story_progress -= 1;
                }
            }
            Interaction::Hovered => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn show_text(
    story_text: Res<main_story_text::GlobalStoryText>,
    progress: Res<progress_status::ProgressStatus>,
    mut textbox: Query<(&mut Text, With<StoryText>)>,
) {
    let text = story_text
        .0
        .get(&progress.main_story)
        .expect("main story not found");
    println!("press pre button");
    for mut textbox in &mut textbox {
        textbox.0.sections[0].value = text[progress.main_story_progress].clone();
    }
}
