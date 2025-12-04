use crate::draw::mesh::MeshObject;
use crate::draw::vertex::Vertex;
use crate::draw::{Shader, Texture};
use crate::error_log;
use asset_importer::mesh;
use asset_importer::mesh::Mesh;
use asset_importer::node::Node;
use asset_importer::postprocess::PostProcessSteps;
use asset_importer::{Importer, Material, Scene, TextureType};
use glam::Vec3;
use std::path::Path;
use image::load;

pub struct Model {
    loaded_textures: Vec<Texture>,
    meshes: Vec<MeshObject>,

    model_path: String,
    gamma_correction: bool,
}

impl Model {
    pub fn new(texture_path: &str, model_path: &str, gamma_correction: bool) -> Model {
        return Model {
            loaded_textures: Vec::new(),
            meshes: Vec::new(),

            model_path: String::from(model_path),
            gamma_correction: gamma_correction,
        };
    }

    pub fn draw(&mut self, shader: &mut Shader) {
        for mesh in self.meshes.iter_mut() {
            mesh.draw(shader);
        }
    }

    fn load_model(&mut self) {
        let importer: Importer = Importer::new();

        let scene = importer
            .read_file(self.model_path.as_str())
            .with_post_process(
                PostProcessSteps::TRIANGULATE
                    | PostProcessSteps::GEN_SMOOTH_NORMALS
                    | PostProcessSteps::FLIP_UVS
                    | PostProcessSteps::CALC_TANGENT_SPACE,
            )
            .import_file(self.model_path.as_str())
            .unwrap_or_else(|e| {
                error_log!("Error loading model:\n{}", e);
                panic!("Error loading model:\n{}", e);
            });
        let home_model_directory = Path::new(self.model_path.as_str())
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        self.process_node(scene.root_node(), &scene);
    }

    fn process_node(&mut self, root: Option<Node>, scene: &Scene) {
        let mut stack = vec![root];
        let mut process_node_meshes = |node: &Node| {
            for i in 0..node.num_meshes() {
                if let Some(mesh) = scene.mesh(i) {
                    self.meshes.push(self.process_mesh(mesh, &scene));
                }
            }
        };

        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                process_node_meshes(&node);

                for i in 0..node.num_children() {
                    stack.push(node.child(i));
                }
            }
        }
    }

    fn process_mesh(&mut self, mesh: Mesh, scene: &Scene) -> MeshObject {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut textures: Vec<Texture> = Vec::new();

        for i in 0..mesh.num_vertices() {
            let mut vertex: Vertex = Vertex::default();

            vertex.position = mesh.vertices()[i];

            if let Some(normals) = &mesh.normals() {
                vertex.normal = normals[i];
            }

            if let Some(texture_coordinates) = &mesh.texture_coords(0) {
                vertex.texture_coordinates = texture_coordinates[i];
            }

            if let Some(tangents) = &mesh.tangents() {
                vertex.tangent = tangents[i];
            }

            if let Some(bitangents) = &mesh.bitangents() {
                vertex.bitangent = bitangents[i];
            }
            vertices.push(vertex);
        }

        for face in mesh.faces() {
            for i in 0..face.num_indices() {
                indices.push(face.indices()[i] as u32);
            }
        }

        let material_index = mesh.material_index();
        let material: Option<Material> = scene.material(material_index);

        // prepare the diffuse maps
        let mut diffuse_maps: Vec<Texture> = self.load_material_textures(
            &material,
            TextureType::Diffuse,
            String::from("texture_diffuse"),
        );
        textures.append(&mut diffuse_maps);



        // prepare specular maps
        let mut specular_maps: Vec<Texture> = self.load_material_textures(
            &material,
            TextureType::Specular,
            String::from("texture_specular"),
        );
        textures.append(&mut specular_maps);

        // prepare normal maps
        let mut normal_maps: Vec<Texture> = self.load_material_textures(
            &material,
            TextureType::Height,
            String::from("texture_normal"),
        );
        textures.append(&mut normal_maps);

        // prepare height maps
        let mut height_maps: Vec<Texture> = self.load_material_textures(
            &material,
            TextureType::Ambient,
            String::from("texture_height"),
        );
        textures.append(&mut height_maps);

        let mesh_object = MeshObject::new(vertices, indices, textures);
        return mesh_object;
    }

    fn load_material_textures(
        &mut self,
        material: &Option<Material>,
        texture_type: TextureType,
        type_name: String,
    ) -> Vec<Texture> {

        let mut textures: Vec<Texture> = Vec::new();
        if let Some(material) = material {
            for i in 0..material.texture_count(texture_type){
                let texture_info = material.texture(texture_type,i).unwrap();
                let mut skip:bool = false;
                for loaded_texture in &self.loaded_textures {
                    if loaded_texture.get_texture_file() == texture_info.path {
                        textures.push(*loaded_texture);
                        skip = true;
                        break;
                    } else {

                    }
                }
                if skip == false {
                    let mut texture = Texture::new(
                        texture_info.path.as_str(),
                        type_name.as_str()
                    ).unwrap_or_else(|e| {
                        error_log!("Error loading texture:\n{}", e);
                        panic!("Error loading texture:\n{}", e);
                    });
                    let _id = texture.create_texture();
                    textures.push(texture.clone());
                    self.loaded_textures.push(texture);

                }

            }
        }


       return textures
    }
}
