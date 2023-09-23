use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

pub mod player;
pub mod camera;
pub mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ThirdPersonCameraPlugin, WorldInspectorPlugin::new()))
        .add_plugins((PlayerPlugin, CameraPlugin, WorldPlugin))
        .run();
}



