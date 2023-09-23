use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_light));
    }
}

fn spawn_light(mut commands: Commands) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                color: Color::rgba_u8(255, 199, 0, 255),
                intensity: 2000.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0., 10., 0.),
            ..default()
        },
        Name::new("Main light"),
    );

    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.))),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
        Name::new("Floor"),
    );

    commands.spawn(floor);
}
