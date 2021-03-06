use std::fs::File;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_orbit_controls::{OrbitCamera, OrbitCameraPlugin};
use std::io::{Read, BufRead};
use std::io;

fn main() -> std::io::Result<()>{
    println!("Input file to visualize ?");
    render_3d_file();
    Ok(())
}

fn render_3d_file() {
    App::build()
        .add_resource(Msaa {samples:4})
        .add_plugins(DefaultPlugins)
        .add_plugin(OrbitCameraPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(convert_file_to_3d.system())
        .run();
}

fn convert_file_to_3d(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> () {
    let mut file_name = String::new();
    io::stdin().lock().read_line(&mut file_name).unwrap();
    let file_name = file_name.trim();
    let file = File::open(file_name).unwrap();
    let mut file_bytes = file.bytes();
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 2. }));
    // convert file to 3d and then create world according to the file
    // we can miss 1 or 2 bytes but it does not matter
    while let (Some(byte_x), Some(byte_y), Some(byte_z)) =
    (file_bytes.next(), file_bytes.next(), file_bytes.next()) {
        match (byte_x, byte_y, byte_z) {
            (Ok(byte_x), Ok(byte_y), Ok(byte_z)) => {
                let (r, g, b) = generate_cube_color(
                    &byte_x, &byte_y, &byte_z);
                let (x_pos, y_pos, z_pos) = generate_cube_position(
                    &byte_x, &byte_y, &byte_z);
                commands
                    .spawn(PbrBundle {
                        mesh: mesh.clone(),
                        material: materials.add(Color::rgb(r, g, b).into()),
                        transform: Transform::from_translation(Vec3::new(x_pos, y_pos, z_pos)),
                        ..Default::default()
                    });
            }
            _ => println!("FAILED")
        }
    }
}

fn setup(
    commands: &mut Commands,
) {
    commands
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(300., 300., 300.))
                .looking_at(Vec3::from((0., 0., 0.)), Vec3::unit_y()),
            ..Default::default()
            // })
        }).with(OrbitCamera::new(400., Vec3::new(150., 150., 150.)))
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 300.0, 0.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 300.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(300.0, 300.0, 0.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(300.0, 300.0, 300.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 300.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 300.0, 300.0)),
            ..Default::default()
        });
}

fn generate_cube_position(x_pos: &u8, y_pos: &u8, z_pos: &u8) -> (f32, f32, f32) {
    const POSITION_FACTOR: f32 = 2.;
   (POSITION_FACTOR * *x_pos as f32, POSITION_FACTOR * *y_pos as f32, POSITION_FACTOR * *z_pos as f32)
}

fn generate_cube_color(x_pos: &u8, y_pos: &u8, z_pos: &u8) -> (f32, f32, f32) {
    const COLOR_FACTOR: f32 = 1./255.;
    (COLOR_FACTOR * *x_pos as f32, COLOR_FACTOR * *y_pos as f32, COLOR_FACTOR * *z_pos as f32)
}
