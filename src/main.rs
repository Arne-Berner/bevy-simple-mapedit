use bevy::window::PrimaryWindow;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Result, Write};
use std::path::Path;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, gizmo_setup))
        .add_systems(Update, (system, render_gizmo))
        .run();
}
#[derive(Resource)]
struct Map(Vec<Vec2>);

#[derive(Resource, Debug)]
struct Holes(Vec<Vec<Vec2>>);

#[derive(Resource)]
struct ActiveHole(bool);

fn setup(mut commands: Commands) {
    commands.insert_resource(Map(vec![]));
    commands.insert_resource(Holes(vec![]));
    commands.insert_resource(ActiveHole(false));
}

fn system(
    mut map: ResMut<Map>,
    mut holes: ResMut<Holes>,
    mut active_hole: ResMut<ActiveHole>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_q.single();
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        if keys.pressed(KeyCode::W) {
            // W is being held down
            if let Some(position) = windows
                .single()
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                map.0.push(position);
            } else {
                println!("Cursor is not in the game window.");
            }
        }
    }
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        if keys.pressed(KeyCode::E) {
            // W is being held down
            if let Some(position) = windows
                .single()
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                if active_hole.0 {
                    let last: isize = (holes.0.len() as isize) - 1;
                    if last >= 0 {
                        holes.0[last as usize].push(position);
                    } else {
                        panic!("Active hole should be initialized!")
                    }
                } else {
                    holes.0.push(vec![position]);
                    active_hole.0 = true;
                }
            } else {
                println!("Cursor is not in the game window.");
            }
        }
    }

    if keys.just_released(KeyCode::E) {
        // Left Button was released
        active_hole.0 = false;
    }

    if keys.pressed(KeyCode::S) {
        // safe holes and map in serde
   let mut f = File::create("saved").expect("hope");
    let buf = serde_json::to_vec(&holes.0).expect("it");
    f.write_all(&buf[..]).expect("works");
    }
    if keys.pressed(KeyCode::L) {
        if let Ok(mut file) = File::open("saved") {
            let mut buf = vec![];
            if file.read_to_end(&mut buf).is_ok() {
                if let Ok(world) = serde_json::from_slice(&buf[..]) {
                    holes.0 = world;
                }
            }
        }
    }
}

fn render_gizmo(map: Res<Map>, holes: Res<Holes>, mut gizmos: Gizmos) {
    for pos in &map.0 {
        gizmos.circle_2d(*pos, 1., Color::GREEN);
    }
    for hole in &holes.0 {
        let len = hole.len();
        if len >= 2 {
            for i in 1..len {
                gizmos.line_2d(hole[i - 1], hole[i], Color::RED)
            }
            gizmos.line_2d(hole[len - 1], hole[0], Color::RED)
        }
    }
}

fn gizmo_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Default,
        },
        ..default()
    });
    commands.spawn(TextBundle::from_section(
        "Hold w to create world points and press e to create holes",
        TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 24.,
            color: Color::WHITE,
        },
    ));
}
