use std::f32::consts::{PI, TAU};

use bevy::{
    app::{App, Plugin, Startup, Update},
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        bundle::Bundle,
        component::Component,
        event::EventReader,
        query::{QuerySingleError, With, Without},
        system::{Commands, Query, Res},
    },
    input::{
        keyboard::KeyCode,
        mouse::{MouseButton, MouseMotion, MouseScrollUnit, MouseWheel},
        ButtonInput,
    },
    math::{EulerRot, Quat, Vec2, Vec3},
    transform::components::Transform,
};

/// A Camera bundle that orbits around a point
#[derive(Bundle, Default)]
pub struct OrbitCam {
    pub camera: Camera3dBundle,
    pub state: OrbitState,
    pub settings: OrbitSettings,
}

/// Settings used by Orbit Camera
#[derive(Component, Debug, Default)]
pub struct OrbitSettings {
    pub orbit_sensitivity: f32,
    pub scroll_wheel_action: ScrollAction,
    pub scroll_sensitivity_line: f32,
    pub scroll_sensitivity_pixel: f32,
    pub orbit_key: Option<KeyCode>,
}

/// Current state of a orbiting camera
#[derive(Component, Debug)]
pub struct OrbitState {
    pub target: Vec3,
    pub radius: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Debug)]
pub enum ScrollAction {
    Zoom,
    VerticalPan,
}

/// Marks the primary camera
#[derive(Component)]
pub struct PrimaryCameraMarker;

/// Marks the Entity that the primary camera should orbit around
#[derive(Component)]
pub struct CameraTarget;

pub struct CameraPlugin;

fn spawn(mut cmds: Commands) {
    cmds.spawn((
        OrbitCam {
            settings: OrbitSettings {
                orbit_sensitivity: 0.01,
                orbit_key: Some(KeyCode::ShiftLeft),
                scroll_sensitivity_line: 0.1,
                ..Default::default()
            },
            ..Default::default()
        },
        PrimaryCameraMarker,
    ));
}

fn update_camera(
    kbd: Res<ButtonInput<KeyCode>>,
    mos: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_event: EventReader<MouseMotion>,
    mouse_scroll_event: EventReader<MouseWheel>,
    mut cam: Query<(&mut OrbitState, &mut Transform, &OrbitSettings), With<PrimaryCameraMarker>>,
    target: Query<&Transform, (With<CameraTarget>, Without<PrimaryCameraMarker>)>,
) {
    // Get the state and transform for the camera
    let (mut state, mut transform, settings) = cam
        .get_single_mut()
        .expect("Multiple or no primary camera");

    // In case of no entity with target marker, allow user to pan camera
    // In case there is a single target marker, set the camera origin to be on that entity
    // In case of multiple target markers, panic
    match target.get_single() {
        Err(QuerySingleError::MultipleEntities(_)) => panic!("There are multiple targets for the primary camera"),
        Err(QuerySingleError::NoEntities(_)) => {},
        Ok(t) => state.target = t.translation,
    };

    // Convert mouse movement and scroll events to Vec2s
    let motion: Vec2 = mouse_motion_event.read().map(|ev| ev.delta).sum();
    let scroll = parse_scroll(mouse_scroll_event, settings);

    // Apply to Pitch/Yaw
    state.orbit(settings, -motion);

    // Apply scroll
    if let ScrollAction::Zoom = settings.scroll_wheel_action {
        state.zoom(scroll.y)
    }

    // Apply transformation
    *transform = state.to_transform();
}

/// Normalize a euler angle so it loops around
fn norm_euler(v: f32) -> f32 {
    ((v + PI) % TAU) - PI
}

fn parse_scroll(mut input: EventReader<MouseWheel>, settings: &OrbitSettings) -> Vec2 {
    let mut result = Vec2::ZERO;

    for ev in input.read() {
        let motion = Vec2 { x: ev.x, y: ev.y };
        let sensitivity = match ev.unit {
            MouseScrollUnit::Line => settings.scroll_sensitivity_line,
            MouseScrollUnit::Pixel => settings.scroll_sensitivity_pixel,
        };

        result += -motion * sensitivity;
    }

    result.exp()
}

impl OrbitState {
    /// Creates a transform for the camera
    fn to_transform(&self) -> Transform {
        let mut transform =
            Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0));
        transform.translation = self.target + transform.back() * self.radius;

        transform
    }

    fn orbit(&mut self, settings: &OrbitSettings, motion: Vec2) {
        let motion = motion * settings.orbit_sensitivity;

        self.yaw = norm_euler(self.yaw + motion.x);
        self.pitch = norm_euler(self.pitch + motion.y);
    }

    fn zoom(&mut self, scroll: f32) {
        if scroll == 0.0 {
            return;
        }
        self.radius *= scroll;
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, update_camera);
    }
}

impl Default for ScrollAction {
    fn default() -> Self {
        Self::Zoom
    }
}

impl Default for OrbitState {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            radius: 10.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}
