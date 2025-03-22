use crate::keybinds::{Keybind, KeybindOptions};
use bevy::{
    app::{App, Plugin, PreUpdate, Startup, Update},
    core_pipeline::core_3d::Camera3d,
    ecs::{
        bundle::Bundle,
        component::Component,
        event::EventReader,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    input::{
        keyboard::KeyCode,
        mouse::{AccumulatedMouseMotion, MouseButton, MouseScrollUnit, MouseWheel},
        ButtonInput,
    },
    math::{EulerRot, Quat, Vec2},
    prelude::Single,
    transform::components::Transform,
};
use std::f32::consts::FRAC_PI_2;

/// A Camera bundle that orbits around a point
#[derive(Bundle, Default)]
pub struct OrbitCam {
    pub camera: Camera3d,
    pub state: OrbitState,
    pub settings: OrbitSettings,
}

/// Settings used by Orbit Camera
#[derive(Component, Debug, Default)]
pub struct OrbitSettings {
    pub orbit_sensitivity: f32,
    pub scroll_sensitivity_line: f32,
    pub scroll_sensitivity_pixel: f32,
    pub move_camera_key: Keybind,
}

/// Current state of Orbit Camera
#[derive(Component, Debug)]
pub struct OrbitState {
    pub distance: f32,
    pub angle: Quat,
}

/// Marks the primary camera
#[derive(Component)]
pub struct PrimaryCamera;

/// Marks the Entity that the primary camera should orbit around
#[derive(Component)]
pub struct CameraTarget;

pub struct CameraPlugin;

/// Spawns an instance of an [OrbitCam] with the [PrimaryCamera] marker
///
/// Note: Multiple [PrimaryCamera] will cause a panic
fn spawn(mut cmds: Commands) {
    cmds.spawn((
        OrbitCam {
            settings: OrbitSettings {
                orbit_sensitivity: 0.01,
                move_camera_key: Keybind(vec![
                    KeybindOptions::MouseButton(MouseButton::Right),
                    KeybindOptions::MouseButton(MouseButton::Left),
                ]),
                scroll_sensitivity_line: 0.1,
                scroll_sensitivity_pixel: 0.01,
            },
            ..Default::default()
        },
        PrimaryCamera,
    ));
}

fn orbit(
    kbd: Res<ButtonInput<KeyCode>>,
    mos: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut cam: Single<(&mut OrbitState, &OrbitSettings), With<PrimaryCamera>>,
) {
    if cam.1.move_camera_key.pressed(&kbd, &mos) {
        let rot = cam.1;
        cam.0.orbit(rot, mouse_motion.delta);
    }
}

/// Updates the camera position
///
/// # Panics
/// This system will panic if:
/// - there is zero or more than one camera with [PrimaryCamera]
/// - there is zero or more than one entity with [CameraTarget]
fn update_camera(
    mut cam: Single<(&mut OrbitState, &mut Transform), With<PrimaryCamera>>,
    target: Single<&Transform, (With<CameraTarget>, Without<PrimaryCamera>)>,
) {
    cam.1.rotation = cam.0.angle;
    cam.1.translation = target.translation - cam.1.forward() * cam.0.distance;
}

/// Calculates the new distance to use for the camera from scroll
fn parse_scroll(
    mut input: EventReader<MouseWheel>,
    mut cam: Query<(&mut OrbitState, &OrbitSettings), With<PrimaryCamera>>,
) {
    let mut result = Vec2::ZERO;

    let mut cam = cam
        .get_single_mut()
        .expect("Found no or too many primary cameras");

    let settings = cam.1;

    for ev in input.read() {
        let motion = Vec2 { x: ev.x, y: ev.y };
        let sensitivity = match ev.unit {
            MouseScrollUnit::Line => settings.scroll_sensitivity_line,
            MouseScrollUnit::Pixel => settings.scroll_sensitivity_pixel,
        };

        result += -motion * sensitivity;
    }

    cam.0.zoom(result.y.exp());
}

impl OrbitState {
    fn orbit(&mut self, settings: &OrbitSettings, motion: Vec2) {
        let motion = motion * settings.orbit_sensitivity;

        let (yaw, pitch, _) = self.angle.to_euler(EulerRot::YXZ);

        let yaw = yaw - motion.x;
        let pitch = (pitch - motion.y).clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01);

        self.angle = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.);
    }
    fn zoom(&mut self, scroll: f32) {
        if scroll == 0.0 {
            return;
        }
        self.distance *= scroll;
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(PreUpdate, (parse_scroll, orbit));
        app.add_systems(Update, update_camera);
    }
}

impl Default for OrbitState {
    fn default() -> Self {
        Self {
            distance: 10.0,
            angle: Quat::IDENTITY,
        }
    }
}
