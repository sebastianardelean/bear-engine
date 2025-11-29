use glam::{Mat4, Vec3};

#[allow(unused)]
pub enum RotationAxis {
    RotationX = 0x00,
    RotationY,
    RotationZ,
}

pub type ScaleMask = u8;

pub const SCALE_X: ScaleMask = 0b001;
pub const SCALE_Y: ScaleMask = 0b010;
pub const SCALE_Z: ScaleMask = 0b100;

#[allow(unused)]
pub fn get_identity() -> Mat4 {
    return Mat4::IDENTITY;
}

#[allow(unused)]
pub fn translate(offset: Vec3) -> Mat4 {
    return Mat4::from_translation(offset);
}

#[allow(unused)]
pub fn scale(factor: f32, axes: ScaleMask) -> Mat4 {
    let x = if axes & SCALE_X != 0 { factor } else { 1.0 };
    let y = if axes & SCALE_Y != 0 { factor } else { 1.0 };
    let z = if axes & SCALE_Z != 0 { factor } else { 1.0 };

    Mat4::from_scale(Vec3::new(x, y, z))
}

#[allow(unused)]
pub fn rotate(angle: f32, rotation_axis: RotationAxis) -> Mat4 {
    return match rotation_axis {
        RotationAxis::RotationX => Mat4::from_rotation_x(angle),
        RotationAxis::RotationY => Mat4::from_rotation_y(angle),
        RotationAxis::RotationZ => Mat4::from_rotation_z(angle),
    };
}
