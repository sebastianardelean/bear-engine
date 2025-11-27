use glam::{Mat4, Quat, Vec3};

pub fn translate() -> Mat4{
    return Mat4::from_translation(Vec3::new(0.5, -0.5, 0.0));


}

pub fn rotate(angle :f32) -> Mat4{
    return Mat4::from_rotation_z(angle);
}

pub fn get_identity() -> Mat4{
    return Mat4::IDENTITY;
}