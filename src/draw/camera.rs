use glam::Vec3;

pub enum CameraMovement {
    Left,
    Right,
    Forward,
    Backward
}

pub const YAW: f32 = -90.0;
pub const PITCH: f32 = 0.0;
pub const SENSITIVITY: f32 = 0.1;
pub const ZOOM: f32 = 45.0;
pub const SPEED: f32 = 2.5;

pub struct Camera {
    position: Vec3,
    front: Vec3,
    up: Vec3,
    right: Vec3,
    world_up: Vec3,
    yaw: f32,
    pitch: f32,
    movement_speed: f32,
    mouse_sensitivity: f32,
    zoom: f32,

}


impl Camera {
    pub fn default(&self) -> Camera {

        return Camera{
            position: Vec3::from([0.0,0.0,0.0]),
            front:Vec3::from([0.0,0.0,-1.0]),
            up: Vec3::from([0.0, 1.0, 0.0]),
            right:Vec3::from([0.0,0.0,0.0]),
            world_up:Vec3::from([0.0, 1.0, 0.0]),
            yaw: YAW,
            pitch: PITCH,
            movement_speed: SPEED,
            mouse_sensitivity:SENSITIVITY,
            zoom:ZOOM

        };
    }

    pub fn new(position: Vec3, up:Vec3, yaw:f32, pitch:f32) -> Camera {

        return Camera{
            position: position,
            front:Vec3::from([0.0,0.0,-1.0]),
            up: up,
            right:Vec3::from([0.0,0.0,0.0]),
            world_up:up,
            yaw: yaw,
            pitch: pitch,
            movement_speed: SPEED,
            mouse_sensitivity:SENSITIVITY,
            zoom:ZOOM
        };
    }

    pub fn update_camera_vectors(&mut self) {
        let mut front:Vec3 =Vec3::from([0.0,0.0,0.0]);
        let yaw = self.yaw.to_radians();
        let pitch = self.pitch.to_radians();

        // Front vector
        self.front = glam::Vec3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        ).normalize();

        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }

    pub fn view_matrix(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(
            self.position,
            self.position + self.front,
            self.up,
        )
    }

    pub fn process_keyboard(&mut self, direction:CameraMovement, delta_time:f32) {
        let velocity:f32 = self.movement_speed * delta_time;
        match direction {
            CameraMovement::Left => {self.position -=self.right*velocity;},
            CameraMovement::Right => {self.position +=self.right*velocity;},
            CameraMovement::Forward => {self.position+=self.front*velocity;},
            CameraMovement::Backward => {self.position-=self.front*velocity;},
        }
    }

    pub fn process_mouse(&mut self, x_offset:f32, y_offset:f32, contrain_pitch :bool) {
        let x_offset_new:f32 = x_offset * self.mouse_sensitivity;
        let y_offset_new:f32 = y_offset * self.mouse_sensitivity;
        self.yaw += x_offset_new;
        self.pitch += y_offset_new;

        if contrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, y_offset:f32){
        self.zoom -=y_offset;
        if (self.zoom < 1.0) {
            self.zoom = 1.0
        }
        if (self.zoom > 45.0) {
            self.zoom = 45.0
        }
    }
}


