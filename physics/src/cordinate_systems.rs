use bevy::math::{Quat, Vec3};

pub struct Global;
pub struct Local;

pub trait ConvertCordinateSystem {
    fn to_global(vec: Vec3, rot: Quat) -> Vec3;
    fn to_local(vec: Vec3, rot: Quat) -> Vec3;
}

impl ConvertCordinateSystem for Global {
    fn to_global(vec: Vec3, rot: Quat) -> Vec3 {
        vec
    }

    fn to_local(vec: Vec3, rot: Quat) -> Vec3 {
        rot.inverse().mul_vec3(vec)
    }
}

impl ConvertCordinateSystem for Local {
    fn to_global(vec: Vec3, rot: Quat) -> Vec3 {
        rot.mul_vec3(vec)
    }

    fn to_local(vec: Vec3, rot: Quat) -> Vec3 {
        vec
    }
}

