use std::f32::consts::{PI, TAU};

use bevy::{
    a11y::accesskit::Point,
    app::{App, Plugin, Startup, Update},
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res},
    },
    input::mouse::MouseMotion,
    log::debug,
    math::{EulerRot, Quat, Vec2, Vec3},
    transform::components::Transform,
};

/// A Camera bundle that orbits around a point
#[derive(Bundle, Default)]
pub struct OrbitCam {
    pub camera: Camera3dBundle,
    pub state: OrbitState,
}

/// Current state of a orbiting camera
#[derive(Component, Debug)]
pub struct OrbitState {
    pub target: OrbitTarget,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
}

/// Targets that can be orbited around
#[derive(Debug)]
pub enum OrbitTarget {
    /// A specific entity
    Entity(Entity),
    /// A set point in space
    Point(Vec3),
}

/// Marks the primary camera
#[derive(Component)]
struct PrimaryCameraMarker;

pub struct CameraPlugin;

fn spawn(mut cmds: Commands) {
    cmds.spawn((OrbitCam::default(), PrimaryCameraMarker));
}

fn update_camera(
    mut mouse_motion_event: EventReader<MouseMotion>,
    mut query: Query<(&mut OrbitState, &mut Transform), With<PrimaryCameraMarker>>,
) {
    // Get the state and transform for the camera
    let (mut state, mut transform) = query
        .get_single_mut()
        .expect("Multiple or no primary camera");

    // Convert movement event to a Vec2
    let mut motion: Vec2 = mouse_motion_event.read().map(|ev| ev.delta).sum();

    // Invert movement axis
    motion.y = -motion.y;
    motion.x = -motion.x;

    // Apply to Pitch/Yaw
    state.yaw = normalize(state.yaw + motion.x * 0.01);
    state.pitch = normalize(state.pitch + motion.y * 0.01);

    // Apply transformation
    *transform = state.to_transform();
}

fn normalize(v: f32) -> f32 {
    if v < -PI {
        return v + TAU;
    }
    if v > PI {
        return v - TAU;
    }
    v
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, update_camera);
    }
}

impl Default for OrbitTarget {
    fn default() -> Self {
        Self::Point(Vec3::ZERO)
    }
}

impl Default for OrbitState {
    fn default() -> Self {
        Self {
            target: OrbitTarget::default(),
            radius: 10.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl OrbitState {
    /// Creates a transform for the camera
    fn to_transform(&self) -> Transform {
        let center = match self.target {
            OrbitTarget::Point(t) => t,
            OrbitTarget::Entity(_) => todo!(),
        };

        let mut transform =
            Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0));
        transform.translation = center + transform.back() * self.radius;

        transform
    }
}
