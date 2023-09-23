use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_object))
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct Speed(f32);

fn player_movement(
    mut player: Query<(&Speed, &mut Transform), With<Player>>,
    camera: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (speed, mut player_transform) in &mut player {
        if let Ok(camera) = camera.get_single() {
            let mut direction = Vec3::ZERO;

            // forward
            if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
                direction += camera.forward();
            }

            // back
            if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
                direction += camera.back();
            }

            // left
            if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
                direction += camera.left();
            }

            // right
            if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
                direction += camera.right();
            }

            direction.y = 0.;
            let movement = direction.normalize_or_zero() * (**speed) * time.delta_seconds();
            player_transform.translation += movement;

            // rotate player to face direction he is currently moving
            if direction.length_squared() > 0. {
                player_transform.look_to(direction, Vec3::Y);
            }
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let flashlight = (
        SpotLightBundle {
            spot_light: SpotLight {
                color: Color::rgba_u8(255, 246, 95, 1),
                intensity: 4000.,
                outer_angle: 0.6,
                inner_angle: 0.5,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0., 0.3, -0.2),
            ..default()
        },
        Name::new("Flashlight"),
    );

    let player = (
        // PbrBundle {
        //     mesh: meshes.add(Mesh::from(shape::Cube::new(1.))),
        //     material: materials.add(Color::BLUE.into()),
        //     transform: Transform::from_xyz(0., 0.5, 0.),
        //     ..default()
        // },
        SceneBundle {
            scene: asset_server.load("Player.gltf#Scene0"),
            transform: Transform::from_xyz(0., 0.5, 0.),
            ..default()
        },
        Speed(2.),
        Player,
        ThirdPersonCameraTarget,
        Name::new("Player"),
    );

    commands.spawn(player).with_children(|parent| {
        parent.spawn(flashlight);
    });
}

fn spawn_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut create_cube = |size: f32, color: Color, translation: Vec3, name: &'static str| {
        (
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::new(size))),
                material: materials.add(color.into()),
                transform: Transform::from_translation(translation),
                ..default()
            },
            Name::new(name),
        )
    };
    let blue_cube = create_cube(1., Color::BLUE, Vec3::new(-3.75, 0.5, -3.75), "Blue Cube");
    let red_cube = create_cube(2., Color::RED, Vec3::new(-3.75, 1., 0.), "Red Cube");
    let purple_cube = create_cube(3., Color::PURPLE, Vec3::new(0., 1.5, -3.75), "Purple Cube");
    let yellow_cube = create_cube(4., Color::YELLOW, Vec3::new(0., 2., 3.75), "Yellow Cube");

    commands.spawn(blue_cube);
    commands.spawn(red_cube);
    commands.spawn(purple_cube);
    commands.spawn(yellow_cube);
}
