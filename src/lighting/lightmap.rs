use crate::voxels::{CHUNK_D, CHUNK_VOL, CHUNK_W};


#[derive(Debug, Clone)]
pub struct Lightmap {
    pub map: Vec<u16>,
}

impl Lightmap {
    pub fn new() -> Self {
        let mut map = vec![0; CHUNK_VOL];
        for value in &mut map {
            *value = 0x0000;
        }
        Lightmap { map }
    }

    pub fn get(&self, x: i32, y: i32, z: i32, channel: i32) -> u8 {
        let index = (y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize;
        ((self.map[index] >> (channel << 2)) & 0xF) as u8
    }
    #[allow(unused)]
    pub fn get_r(&self, x: i32, y: i32, z: i32) -> u8 {
        let index = (y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize;
        (self.map[index] & 0xF) as u8
    }

    #[allow(unused)]
    pub fn get_g(&self, x: i32, y: i32, z: i32) -> u8 {
        let index = (y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize;
        ((self.map[index] >> 4) & 0xF) as u8
    }

    #[allow(unused)]
    pub fn get_b(&self, x: i32, y: i32, z: i32) -> u8 {
        let index = (y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize;
        ((self.map[index] >> 8) & 0xF) as u8
    }

    #[allow(unused)]
    pub fn get_s(&self, x: i32, y: i32, z: i32) -> u8 {
        let index = (y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize;
        ((self.map[index] >> 12) & 0xF) as u8
    }

    #[allow(unused)]
    pub fn set_r(&mut self, x: i32, y: i32, z: i32, value: i32) {
        let index = (y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize;
        self.map[index] = (self.map[index] & 0xFFF0) | (value as u16);
    }

    #[allow(unused)]
    pub fn set_g(&mut self, x: i32, y: i32, z: i32, value: i32) {
        let index = (y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize;
        self.map[index] = (self.map[index] & 0xFF0F) | ((value << 4) as u16);
    }

    #[allow(unused)]
    pub fn set_b(&mut self, x: i32, y: i32, z: i32, value: i32) {
        let index = (y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize;
        self.map[index] = (self.map[index] & 0xF0FF) | ((value << 8) as u16);
    }

    pub fn set_s(&mut self, x: i32, y: i32, z: i32, value: i32) {
        let index = (y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize;
        self.map[index] = (self.map[index] & 0x0FFF) | ((value << 12) as u16);
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32, channel: i32, value: i32) {
        let index = (y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize;
        self.map[index] = (self.map[index] & (0xFFFF & (!(0xF << (channel * 4))))) | ((value << (channel << 2)) as u16);
    }
}
