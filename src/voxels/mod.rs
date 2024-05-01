use noise::{NoiseFn, OpenSimplex};

use crate::lighting::lightmap::Lightmap;

pub mod chunks;

#[derive(Clone, Copy, Debug)]
pub struct Voxel {
    pub id: u8,
}

pub const CHUNK_W: i32 = 16;
pub const CHUNK_H: i32 = 16;
pub const CHUNK_D: i32 = 16;
pub const CHUNK_VOL: usize = (CHUNK_W * CHUNK_H * CHUNK_D) as usize;

#[derive(Clone)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub modified: bool,
    pub voxels: [Voxel; CHUNK_VOL],
    pub lightmap: Lightmap 
}

impl Chunk {
    pub fn new(x_pos: i32, y_pos: i32, z_pos: i32) -> Self {
        let mut voxels = [Voxel {id: 0}; CHUNK_VOL];
        let perlin = OpenSimplex::new(1);
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let real_x = x + x_pos * CHUNK_W ;
                let real_z = z + z_pos * CHUNK_D ;
                //let height = perlin.get([(x as f64) * 0.0125, (z as f64) * 0.0125]);
                for y in 0..CHUNK_H {
                    let real_y = y + y_pos * CHUNK_H ;
                    let id = perlin.get([(real_x as f64) * 0.0125, (real_y as f64) * 0.0125, (real_z as f64) * 0.0125]) > 0.1;
                    let chunk_index = ((y * CHUNK_D + z) * CHUNK_W + x) as usize;
                    if real_y <= 2 {
                        voxels[chunk_index].id = 2;
                    } else {
                        voxels[chunk_index].id = id as u8;
                    }
                }
            }
        }
        Chunk { x: x_pos, y: y_pos, z: z_pos, modified: true, voxels, lightmap: Lightmap::new() }
    }
}


// Block

const BLOCK_COUNT: usize = 256;

#[derive(Clone)]
pub struct Block {
    pub id: u32,
    pub texture_faces: [i32; 6],
    pub emission: [u8; 3],
    pub draw_group: u8,
    pub light_passing: bool,
}

pub struct BlockRegistry {
    pub blocks: Vec<Option<Block>>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        Self { blocks: vec![None;BLOCK_COUNT]  }
    } 
    pub fn get(&self, id: u8) -> Option<&Block> {
        self.blocks[id as usize].as_ref()
    }
}

impl Block {
    pub fn new(id: u32, texture: i32) -> Self {
        Self {
            id,
            texture_faces: [texture; 6],
            emission: [0; 3],
            draw_group: 0,
            light_passing: false,
        }
    }
}