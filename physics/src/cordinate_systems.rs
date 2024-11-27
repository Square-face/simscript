use bevy::math::{Mat3, Quat, Vec3};

pub struct Global;
pub struct Local;

pub trait ConvertCordinateSystem {
    fn vec3_to_global(vec: Vec3, rot: Quat) -> Vec3;
    fn vec3_to_local(vec: Vec3, rot: Quat) -> Vec3;

    fn mat3_to_global(mat: Mat3, rot: Quat) -> Mat3;
    fn mat3_to_local(mat: Mat3, rot: Quat) -> Mat3;
}

impl ConvertCordinateSystem for Global {
    fn vec3_to_global(vec: Vec3, _: Quat) -> Vec3 {
        vec
    }

    fn vec3_to_local(vec: Vec3, rot: Quat) -> Vec3 {
        rot.inverse().mul_vec3(vec)
    }

    fn mat3_to_global(mat: Mat3, _: Quat) -> Mat3 {
        mat
    }

    fn mat3_to_local(mat: Mat3, rot: Quat) -> Mat3 {
        let rot = Mat3::from_quat(rot.inverse());
        mat.mul_mat3(&rot)
    }
}

impl ConvertCordinateSystem for Local {
    fn vec3_to_global(mat: Vec3, rot: Quat) -> Vec3 {
        rot.mul_vec3(mat)
    }

    fn vec3_to_local(vec: Vec3, _: Quat) -> Vec3 {
        vec
    }

    fn mat3_to_global(mat: Mat3, rot: Quat) -> Mat3 {
        let rot = Mat3::from_quat(rot);
        mat.mul_mat3(&rot)
    }

    fn mat3_to_local(mat: Mat3, _: Quat) -> Mat3 {
        mat
    }
}
