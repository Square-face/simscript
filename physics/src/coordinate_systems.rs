use std::marker::Send;

use glam::{Mat3, Quat, Vec3};

#[cfg(feature = "bevy_components")]
use bevy::prelude::Component;

#[cfg_attr(
    not(feature = "bevy_components"),
    derive(Debug, PartialEq, Clone, Copy)
)]
#[cfg_attr(
    feature = "bevy_components",
    derive(Component, Debug, PartialEq, Clone, Copy)
)]
pub struct Global;
#[cfg_attr(
    not(feature = "bevy_components"),
    derive(Debug, PartialEq, Clone, Copy)
)]
#[cfg_attr(
    feature = "bevy_components",
    derive(Component, Debug, PartialEq, Clone, Copy)
)]
pub struct Local;

pub trait CoordinateConvert {
    type Global;
    type Local;

    #[must_use]
    fn to_global(self, rot: Quat) -> Self::Global;
    #[must_use]
    fn to_local(self, rot: Quat) -> Self::Local;
}

pub trait CoordinateSystem: Copy + Send + Sync + 'static {
    #[must_use]
    fn vec3_to_global(vec: Vec3, rot: Quat) -> Vec3;
    #[must_use]
    fn vec3_to_local(vec: Vec3, rot: Quat) -> Vec3;

    #[must_use]
    fn quat_to_global(quat: Quat, rot: Quat) -> Quat;
    #[must_use]
    fn quat_to_local(quat: Quat, rot: Quat) -> Quat;

    #[must_use]
    fn mat3_to_global(mat: Mat3, rot: Quat) -> Mat3;
    #[must_use]
    fn mat3_to_local(mat: Mat3, rot: Quat) -> Mat3;
}

impl CoordinateSystem for Global {
    #[inline]
    fn vec3_to_global(vec: Vec3, _: Quat) -> Vec3 {
        vec
    }

    #[inline]
    fn vec3_to_local(vec: Vec3, rot: Quat) -> Vec3 {
        rot.inverse().mul_vec3(vec)
    }

    #[inline]
    fn mat3_to_global(mat: Mat3, _: Quat) -> Mat3 {
        mat
    }

    #[inline]
    fn mat3_to_local(mat: Mat3, rot: Quat) -> Mat3 {
        let rot = Mat3::from_quat(rot.inverse());
        mat.mul_mat3(&rot)
    }

    #[inline]
    fn quat_to_global(quat: Quat, _: Quat) -> Quat {
        quat
    }

    #[inline]
    fn quat_to_local(quat: Quat, rot: Quat) -> Quat {
        rot.inverse().mul_quat(quat)
    }
}

impl CoordinateSystem for Local {
    #[inline]
    fn vec3_to_global(mat: Vec3, rot: Quat) -> Vec3 {
        rot.mul_vec3(mat)
    }

    #[inline]
    fn vec3_to_local(vec: Vec3, _: Quat) -> Vec3 {
        vec
    }

    #[inline]
    fn mat3_to_global(mat: Mat3, rot: Quat) -> Mat3 {
        let rot = Mat3::from_quat(rot);
        mat.mul_mat3(&rot)
    }

    #[inline]
    fn mat3_to_local(mat: Mat3, _: Quat) -> Mat3 {
        mat
    }

    #[inline]
    fn quat_to_global(quat: Quat, rot: Quat) -> Quat {
        rot.mul_quat(quat)
    }

    #[inline]
    fn quat_to_local(quat: Quat, _: Quat) -> Quat {
        quat
    }
}
