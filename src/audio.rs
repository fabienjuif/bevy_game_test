use bevy::prelude::*;
use bevy_cameraman::Cameraman;

const DEBUG: bool = false;
const MUSIC_AUDIO_ID: &str = "sounds/music.ogg";

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
    }
}

fn setup(
    mut commands: Commands,
    server: Res<AssetServer>,
    query_camera: Query<Entity, With<Cameraman>>,
) {
    let gap = 200.;
    let listener = SpatialListener::new(gap);

    commands.spawn(AudioBundle {
        source: server.load(MUSIC_AUDIO_ID),
        settings: PlaybackSettings::LOOP.with_spatial(false),
    });

    for entity in &query_camera {
        let mut cmd = commands.entity(entity);
        let cmd = cmd.insert((SpatialBundle::default(), listener.clone()));

        if DEBUG {
            cmd.with_children(|parent| {
                // left ear
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::splat(20.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-gap / 2.0, 0.0, 0.0),
                    ..default()
                });

                // right ear
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::splat(20.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(gap / 2.0, 0.0, 0.0),
                    ..default()
                });
            });
        }
    }
}
