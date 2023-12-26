use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use rand::seq::SliceRandom;

pub mod config;
pub mod wave;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tilemap_image = asset_server.load("Dungeon_Tileset - Copy.png");
    let tilemap = TextureAtlas::from_grid(tilemap_image, Vec2::new(16.0, 16.0), 6, 6, None, None);
    let tilemap_handle = texture_atlases.add(tilemap);

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::LIME_GREEN),
        },
        ..default()
    });

    for y in 0..=10 {
        for x in 0..=14 {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: tilemap_handle.clone(),
                sprite: TextureAtlasSprite::new(
                    (0..=35)
                        .collect::<Vec<usize>>()
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .to_owned(),
                ),
                transform: Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(
                    -WINDOW_WIDTH / 2.0 + (16 * 3 * x) as f32,
                    WINDOW_HEIGHT / 2.0 - (16 * 3 * y) as f32,
                    0.0,
                )),
                ..default()
            });
        }
    }
}

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Wave Function Collapse".into(),
                        resizable: false,
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
