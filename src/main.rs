use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};
use dotenv::dotenv;

mod config;

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
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup)
        .add_system(story_text_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(360., 540., 100.)),
        ..default()
    });

    let font = asset_server.load("fonts/DungGeunMo.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

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
                        sections: vec![],
                        alignment: TextAlignment::Center,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
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
}

#[derive(Component)]
struct StoryText;

fn story_text_system(time: Res<Time>, mut query: Query<(&mut Text, With<StoryText>)>) {
    let story: String = "이건 나의 몫이고 저건 너의 몫이다".to_string();
    for text in &mut query {
        let seconds = time.elapsed_seconds();
        println!("{}", seconds);
    }
}

// let button_entity = commands
//     .spawn(NodeBundle {
//         style: Style {
//             // center button
//             size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//             justify_content: JustifyContent::Center,
//             align_items: AlignItems::Center,
//             ..default()
//         },
//         ..default()
//     })
//     .with_children(|parent| {
//         parent
//             .spawn(ButtonBundle {
//                 style: Style {
//                     size: Size::new(Val::Px(150.0), Val::Px(65.0)),
//                     // horizontally center child text
//                     justify_content: JustifyContent::Center,
//                     // vertically center child text
//                     align_items: AlignItems::Center,
//                     ..default()
//                 },
//                 background_color: Color::rgb(0.15, 0.15, 0.15).into(),
//                 ..default()
//             })
//             .with_children(|parent| {
//                 parent.spawn(TextBundle::from_section(
//                     "Play",
//                     TextStyle {
//                         font: asset_server.load("fonts/DungGeunMo.ttf"),
//                         font_size: 40.0,
//                         color: Color::rgb(0.9, 0.9, 0.9),
//                         ..default()
//                     },
//                 ));
//             });
//     })
//     .id();
