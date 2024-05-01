use glam::{Mat4, Vec3, Quat};

pub struct Camera {
    pub position: Vec3,
    pub fov: f32,
    pub rotation: Quat,
    pub front: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, fov: f32) -> Self {
        let rotation = Quat::IDENTITY;
        let front = rotation.mul_vec3(Vec3::Z);
        let right = rotation.mul_vec3(Vec3::X);
        let up = rotation.mul_vec3(Vec3::Y);
        
        Self { position, fov, rotation, front, right, up }
    }

    fn update_vectors(&mut self) {
        self.front = self.rotation.mul_vec3(Vec3::Z);
        self.right = self.rotation.mul_vec3(Vec3::X);
        self.up = self.rotation.mul_vec3(Vec3::Y);
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation *= Quat::from_rotation_z(z) * Quat::from_rotation_y(y) * Quat::from_rotation_x(x);
        self.update_vectors();
    }

    pub fn get_projection(&self, width: f32, height: f32) -> Mat4 {
        let aspect = width / height;
        Mat4::perspective_rh(self.fov, aspect, 0.1, 1500.0)
    }

    pub fn get_view(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }
}
