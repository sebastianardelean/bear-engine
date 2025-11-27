use glam::{Mat4, Quat, Vec3};

pub enum RotationAxis {
    ROTATION_X,
    ROTATION_Y,
    ROTATION_Z,
}

pub type ScaleMask = u8;

pub const SCALE_X: ScaleMask = 0b001;
pub const SCALE_Y: ScaleMask = 0b010;
pub const SCALE_Z: ScaleMask = 0b100;

pub fn get_identity() -> Mat4 {
    return Mat4::IDENTITY;
}
pub fn translate(offset: Vec3) -> Mat4 {
    return Mat4::from_translation(offset);
}

pub fn scale(factor: f32, axes: ScaleMask) -> Mat4 {
    let x = if axes & SCALE_X != 0 { factor } else { 1.0 };
    let y = if axes & SCALE_Y != 0 { factor } else { 1.0 };
    let z = if axes & SCALE_Z != 0 { factor } else { 1.0 };

    Mat4::from_scale(Vec3::new(x, y, z))
}

pub fn rotate(angle: f32, rotation_axis: RotationAxis) -> Mat4 {
    return match rotation_axis {
        RotationAxis::ROTATION_X => Mat4::from_rotation_x(angle),
        RotationAxis::ROTATION_Y => Mat4::from_rotation_y(angle),
        RotationAxis::ROTATION_Z => Mat4::from_rotation_z(angle),
    };
}
