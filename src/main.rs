use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tilemap_image = asset_server.load("Dungeon_Tileset.png");
    let tilemap = TextureAtlas::from_grid(tilemap_image, Vec2::new(16.0, 16.0), 7, 1, None, None);
    let tilemap_handle = texture_atlases.add(tilemap);

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GREEN),
        },
        ..default()
    });

    commands.spawn(SpriteSheetBundle {
        texture_atlas: tilemap_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_scale(Vec3::splat(3.0)),
        ..default()
    });
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
                        ..default()
                    }),
                    ..default()
                }),
        )
        // .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
