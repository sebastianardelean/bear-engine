use glam::Vec3;

const MAX_BONE_INFLUENCE:usize = 4;
pub struct Vertex {
    pub position:Vec3,
    pub normal:Vec3,
    pub texture_coordinates:Vec3,
    pub tangent:Vec3,
    pub bitangent:Vec3,
    pub bone_ids:[i32;MAX_BONE_INFLUENCE],
    pub weights:[f32;MAX_BONE_INFLUENCE],
}
impl Vertex {
    pub fn default() -> Vertex {
        return Vertex{
            position:Vec3::default(),
            normal:Vec3::default(),
            texture_coordinates:Vec3::default(),
            tangent:Vec3::default(),
            bitangent:Vec3::default(),
            bone_ids:[0;MAX_BONE_INFLUENCE],
            weights:[0.0;MAX_BONE_INFLUENCE],
        }
    }
}
