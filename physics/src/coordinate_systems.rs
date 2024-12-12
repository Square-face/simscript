use glam::{Mat3, Quat, Vec3};

#[derive(PartialEq, Debug)]
pub struct Global;
#[derive(PartialEq, Debug)]
pub struct Local;

pub trait CoordinateConvert {
    type Global;
    type Local;

    #[must_use]
    fn to_global(self, rot: Quat) -> Self::Global;
    #[must_use]
    fn to_local(self, rot: Quat) -> Self::Local;
}

pub trait CoordinateSystem {
    #[must_use]
    fn vec3_to_global(vec: Vec3, rot: Quat) -> Vec3;
    #[must_use]
    fn vec3_to_local(vec: Vec3, rot: Quat) -> Vec3;

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
}
