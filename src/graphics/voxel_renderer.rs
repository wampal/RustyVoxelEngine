use crate::voxels::{ BlockRegistry, Chunk, Voxel, CHUNK_D, CHUNK_H, CHUNK_W };

use super::mesh::Mesh;

const VERTEX_SIZE: usize = 3 +2 + 4;

fn cdiv(x: i32, a: i32) -> i32 {
    if x < 0 { x / a - 1 } else { x / a }
}

fn local_neg(x: i32, size: i32) -> i32 {
    if x < 0 { size + x } else { x }
}

fn local(x: i32, size: i32) -> i32 {
    if x >= size { x - size } else { local_neg(x, size) }
}

fn is_chunk(x: i32, y: i32, z: i32, chunks: &[Option<Chunk>]) -> bool {
    get_chunk(x, y, z, chunks).is_some()
}

fn get_chunk(x: i32, y: i32, z: i32, chunks: &[Option<Chunk>]) -> Option<&Chunk> {
    let index =
        ((cdiv(y, CHUNK_H) + 1) * 3 + cdiv(z, CHUNK_D) + 1) * 3 +
        cdiv(x, CHUNK_W) +
        1;
    if index >= 0 && index < chunks.len() as i32{
        chunks[index as usize].as_ref()
    } else {
        None
    }
}

fn light(x: i32, y: i32, z: i32, channel: i32, chunks: &[Option<Chunk>]) -> u8 {
    if let Some(chunk) = get_chunk(x, y, z, chunks) {
        chunk.lightmap.get(
            local(x, CHUNK_W),
            local(y, CHUNK_H),
            local(z, CHUNK_D),
            channel
        )
    } else {
        0
    }
}

fn voxel(x: i32, y: i32, z: i32, chunks: &[Option<Chunk>]) -> Option<&Voxel> {
    if let Some(chunk) = get_chunk(x, y, z, chunks) {
        let lx = local(x, CHUNK_W) as usize;
        let ly = local(y, CHUNK_H) as usize;
        let lz = local(z, CHUNK_D) as usize;
        chunk.voxels.get((ly * CHUNK_D as usize + lz) * CHUNK_W as usize + lx)
    } else {
        None
    }
}

fn is_blocked(x: i32, y: i32, z: i32, blocks: &BlockRegistry, group: u8, chunks: &[Option<Chunk>]) -> bool {
    if !is_chunk(x, y, z, chunks) {
        return true;
    }

    let voxel = voxel(x, y, z, chunks);
    match voxel {
        Some(voxel) => {
            let block_id = voxel.id;
            if block_id != 0 {
                let local_block = blocks.blocks.get(block_id as usize);
                if let Some(Some(local_block)) = local_block {
                    local_block.draw_group == group
                } else {
                    false
                }
            } else {
                false
            }
        }
        None => false,
    }
}


fn vertex(
    buffer: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
    u: f32,
    v: f32,
    r: f32,
    g: f32,
    b: f32,
    s: f32
) {
    buffer.push(x);
    buffer.push(y);
    buffer.push(z);
    buffer.push(u);
    buffer.push(v);
    buffer.push(r);
    buffer.push(g);
    buffer.push(b);
    buffer.push(s);
}

fn setup_uv(index: usize, uvsize: f32) -> (f32, f32, f32, f32) {
        let u1 = ((index % 16) as f32) * uvsize;
        let v1 = 1.0 - ((1 + index / 16) as f32) * uvsize;
        let u2 = u1 + uvsize;
        let v2 = v1 + uvsize;
        (u1, v1, u2, v2)

}

pub struct VoxelRenderer {
    buffer: Vec<f32>,
}

impl VoxelRenderer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity * VERTEX_SIZE * 6),
        }
    }

    pub fn render(
        &mut self,
        chunk: &Chunk,
        chunks: &Vec<Option<Chunk>>,
        blocks: &BlockRegistry
    ) -> Mesh {
        self.buffer.clear();

        for y in 0_..CHUNK_H {
            for z in 0_..CHUNK_D {
                for x in 0..CHUNK_W {
                    let voxel = &chunk.voxels[(y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize];
                    let id = voxel.id;

                    if id == 0 {
                        continue;
                    }

                    
                    let mut l;
                    let uvsize = 1.0 / 16.0;

                    let block = blocks.blocks.get(id as usize);
                    
                    if let Some(Some(block)) = block {
                    let group = block.draw_group;
                    if !is_blocked(x,y+1,z, &blocks, group, &chunks){
                        //l = 1.0_f32;

                        let (u1, v1, u2, v2) = setup_uv(block.texture_faces[3] as usize, uvsize);
    
                        let lr = light(x,y+1,z, 0, &chunks) as f32 / 15.0_f32;
                        let lg = light(x,y+1,z, 1, &chunks) as f32 / 15.0_f32;
                        let lb = light(x,y+1,z, 2, &chunks) as f32 / 15.0_f32;
                        let ls = light(x,y+1,z, 3, &chunks) as f32 / 15.0_f32;
    
                        let lr0 = (light(x-1,y+1,z,0, &chunks) as f32 + lr*30_f32 + light(x-1,y+1,z-1,0, &chunks) as f32 + light(x,y+1,z-1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr1 = (light(x-1,y+1,z,0, &chunks) as f32 + lr*30_f32 + light(x-1,y+1,z+1,0, &chunks) as f32 + light(x,y+1,z+1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr2 = (light(x+1,y+1,z,0, &chunks) as f32 + lr*30_f32 + light(x+1,y+1,z+1,0, &chunks) as f32 + light(x,y+1,z+1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr3 = (light(x+1,y+1,z,0, &chunks) as f32 + lr*30_f32 + light(x+1,y+1,z-1,0, &chunks) as f32 + light(x,y+1,z-1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lg0 = (light(x-1,y+1,z,1, &chunks) as f32 + lg*30_f32 + light(x-1,y+1,z-1,1, &chunks) as f32 + light(x,y+1,z-1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg1 = (light(x-1,y+1,z,1, &chunks) as f32 + lg*30. + light(x-1,y+1,z+1,1, &chunks) as f32 + light(x,y+1,z+1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg2 = (light(x+1,y+1,z,1, &chunks) as f32 + lg*30. + light(x+1,y+1,z+1,1, &chunks) as f32 + light(x,y+1,z+1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg3 = (light(x+1,y+1,z,1, &chunks) as f32 + lg*30. + light(x+1,y+1,z-1,1, &chunks) as f32 + light(x,y+1,z-1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lb0 = (light(x-1,y+1,z,2, &chunks) as f32 + lb*30. + light(x-1,y+1,z-1,2, &chunks) as f32 + light(x,y+1,z-1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb1 = (light(x-1,y+1,z,2, &chunks) as f32 + lb*30. + light(x-1,y+1,z+1,2, &chunks) as f32 + light(x,y+1,z+1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb2 = (light(x+1,y+1,z,2, &chunks) as f32 + lb*30. + light(x+1,y+1,z+1,2, &chunks) as f32 + light(x,y+1,z+1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb3 = (light(x+1,y+1,z,2, &chunks) as f32 + lb*30. + light(x+1,y+1,z-1,2, &chunks) as f32 + light(x,y+1,z-1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let ls0 = (light(x-1,y+1,z,3, &chunks) as f32 + ls*30. + light(x-1,y+1,z-1,3, &chunks) as f32 + light(x,y+1,z-1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls1 = (light(x-1,y+1,z,3, &chunks) as f32 + ls*30. + light(x-1,y+1,z+1,3, &chunks) as f32 + light(x,y+1,z+1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls2 = (light(x+1,y+1,z,3, &chunks) as f32 + ls*30. + light(x+1,y+1,z+1,3, &chunks) as f32 + light(x,y+1,z+1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls3 = (light(x+1,y+1,z,3, &chunks) as f32 + ls*30. + light(x+1,y+1,z-1,3, &chunks) as f32 + light(x,y+1,z-1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32+0.5_f32, z as f32-0.5_f32, u2,v1, lr0, lg0, lb0, ls0);                         
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32+0.5_f32, z as f32+0.5_f32, u2,v2, lr1, lg1, lb1, ls1);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32+0.5_f32, z as f32+0.5_f32, u1,v2, lr2, lg2, lb2, ls2);                              
                        
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32+0.5_f32, z as f32-0.5_f32, u2,v1, lr0, lg0, lb0, ls0);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32+0.5_f32, z as f32+0.5_f32, u1,v2, lr2, lg2, lb2, ls2);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32+0.5_f32, z as f32-0.5_f32, u1,v1, lr3, lg3, lb3, ls3);
                    }
                }

                    if let Some(Some(block)) = block {
                    let group = block.draw_group;
                    if !is_blocked(x,y-1,z, &blocks, group, &chunks){
                        //l = 0.75_f32;

                        
                        let (u1, v1, u2, v2) = setup_uv(block.texture_faces[2] as usize, uvsize);
    
                        let lr = light(x,y-1,z, 0, &chunks) as f32 / 15.0_f32;
                        let lg = light(x,y-1,z, 1, &chunks) as f32 / 15.0_f32;
                        let lb = light(x,y-1,z, 2, &chunks) as f32 / 15.0_f32;
                        let ls = light(x,y-1,z, 3, &chunks) as f32 / 15.0_f32;
    
                        let lr0 = (light(x-1,y-1,z-1,0, &chunks) as f32 + lr*30. + light(x-1,y-1,z,0, &chunks) as f32 + light(x,y-1,z-1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr1 = (light(x+1,y-1,z+1,0, &chunks) as f32 + lr*30. + light(x+1,y-1,z,0, &chunks) as f32 + light(x,y-1,z+1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr2 = (light(x-1,y-1,z+1,0, &chunks) as f32 + lr*30. + light(x-1,y-1,z,0, &chunks) as f32 + light(x,y-1,z+1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr3 = (light(x+1,y-1,z-1,0, &chunks) as f32 + lr*30. + light(x+1,y-1,z,0, &chunks) as f32 + light(x,y-1,z-1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lg0 = (light(x-1,y-1,z-1,1, &chunks) as f32 + lg*30. + light(x-1,y-1,z,1, &chunks) as f32 + light(x,y-1,z-1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg1 = (light(x+1,y-1,z+1,1, &chunks) as f32 + lg*30. + light(x+1,y-1,z,1, &chunks) as f32 + light(x,y-1,z+1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg2 = (light(x-1,y-1,z+1,1, &chunks) as f32 + lg*30. + light(x-1,y-1,z,1, &chunks) as f32 + light(x,y-1,z+1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg3 = (light(x+1,y-1,z-1,1, &chunks) as f32 + lg*30. + light(x+1,y-1,z,1, &chunks) as f32 + light(x,y-1,z-1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lb0 = (light(x-1,y-1,z-1,2, &chunks) as f32 + lb*30. + light(x-1,y-1,z,2, &chunks) as f32 + light(x,y-1,z-1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb1 = (light(x+1,y-1,z+1,2, &chunks) as f32 + lb*30. + light(x+1,y-1,z,2, &chunks) as f32 + light(x,y-1,z+1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb2 = (light(x-1,y-1,z+1,2, &chunks) as f32 + lb*30. + light(x-1,y-1,z,2, &chunks) as f32 + light(x,y-1,z+1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb3 = (light(x+1,y-1,z-1,2, &chunks) as f32 + lb*30. + light(x+1,y-1,z,2, &chunks) as f32 + light(x,y-1,z-1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let ls0 = (light(x-1,y-1,z-1,3, &chunks) as f32 + ls*30. + light(x-1,y-1,z,3, &chunks) as f32 + light(x,y-1,z-1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls1 = (light(x+1,y-1,z+1,3, &chunks) as f32 + ls*30. + light(x+1,y-1,z,3, &chunks) as f32 + light(x,y-1,z+1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls2 = (light(x-1,y-1,z+1,3, &chunks) as f32 + ls*30. + light(x-1,y-1,z,3, &chunks) as f32 + light(x,y-1,z+1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls3 = (light(x+1,y-1,z-1,3, &chunks) as f32 + ls*30. + light(x+1,y-1,z,3, &chunks) as f32 + light(x,y-1,z-1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32-0.5_f32, z as f32-0.5_f32, u1,v1, lr0,lg0,lb0,ls0);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32-0.5_f32, z as f32+0.5_f32, u2,v2, lr1,lg1,lb1,ls1);                         
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32-0.5_f32, z as f32+0.5_f32, u1,v2, lr2,lg2,lb2,ls2);                              

                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32-0.5_f32, z as f32-0.5_f32, u1,v1, lr0,lg0,lb0,ls0);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32-0.5_f32, z as f32-0.5_f32, u2,v1, lr3,lg3,lb3,ls3);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32-0.5_f32, z as f32+0.5_f32, u2,v2, lr1,lg1,lb1,ls1);
                    }
                }
    
                    if let Some(Some(block)) = block {
                    let group = block.draw_group;
                    if !is_blocked(x+1,y,z, &blocks, group,&chunks){
                        //l = 0.95_f32;

                        let (u1, v1, u2, v2) = setup_uv(block.texture_faces[1] as usize, uvsize);
    
                        let lr = light(x+1,y,z, 0, &chunks) as f32 / 15.0_f32;
                        let lg = light(x+1,y,z, 1, &chunks) as f32 / 15.0_f32;
                        let lb = light(x+1,y,z, 2, &chunks) as f32 / 15.0_f32;
                        let ls = light(x+1,y,z, 3, &chunks) as f32 / 15.0_f32;
    
                        let lr0 = (light(x+1,y-1,z-1,0, &chunks) as f32 + lr*30. + light(x+1,y,z-1,0, &chunks) as f32 + light(x+1,y-1,z,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr1 = (light(x+1,y+1,z-1,0, &chunks) as f32 + lr*30. + light(x+1,y,z-1,0, &chunks) as f32 + light(x+1,y+1,z,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr2 = (light(x+1,y+1,z+1,0, &chunks) as f32 + lr*30. + light(x+1,y,z+1,0, &chunks) as f32 + light(x+1,y+1,z,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr3 = (light(x+1,y-1,z+1,0, &chunks) as f32 + lr*30. + light(x+1,y,z+1,0, &chunks) as f32 + light(x+1,y-1,z,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lg0 = (light(x+1,y-1,z-1,1, &chunks) as f32 + lg*30. + light(x+1,y,z-1,1, &chunks) as f32 + light(x+1,y-1,z,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg1 = (light(x+1,y+1,z-1,1, &chunks) as f32 + lg*30. + light(x+1,y,z-1,1, &chunks) as f32 + light(x+1,y+1,z,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg2 = (light(x+1,y+1,z+1,1, &chunks) as f32 + lg*30. + light(x+1,y,z+1,1, &chunks) as f32 + light(x+1,y+1,z,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg3 = (light(x+1,y-1,z+1,1, &chunks) as f32 + lg*30. + light(x+1,y,z+1,1, &chunks) as f32 + light(x+1,y-1,z,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lb0 = (light(x+1,y-1,z-1,2, &chunks) as f32 + lb*30. + light(x+1,y,z-1,2, &chunks) as f32 + light(x+1,y-1,z,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb1 = (light(x+1,y+1,z-1,2, &chunks) as f32 + lb*30. + light(x+1,y,z-1,2, &chunks) as f32 + light(x+1,y+1,z,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb2 = (light(x+1,y+1,z+1,2, &chunks) as f32 + lb*30. + light(x+1,y,z+1,2, &chunks) as f32 + light(x+1,y+1,z,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb3 = (light(x+1,y-1,z+1,2, &chunks) as f32 + lb*30. + light(x+1,y,z+1,2, &chunks) as f32 + light(x+1,y-1,z,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let ls0 = (light(x+1,y-1,z-1,3, &chunks) as f32 + ls*30. + light(x+1,y,z-1,3, &chunks) as f32 + light(x+1,y-1,z,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls1 = (light(x+1,y+1,z-1,3, &chunks) as f32 + ls*30. + light(x+1,y,z-1,3, &chunks) as f32 + light(x+1,y+1,z,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls2 = (light(x+1,y+1,z+1,3, &chunks) as f32 + ls*30. + light(x+1,y,z+1,3, &chunks) as f32 + light(x+1,y+1,z,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls3 = (light(x+1,y-1,z+1,3, &chunks) as f32 + ls*30. + light(x+1,y,z+1,3, &chunks) as f32 + light(x+1,y-1,z,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32-0.5_f32, z as f32-0.5_f32, u2,v1, lr0,lg0,lb0,ls0);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32+0.5_f32, z as f32-0.5_f32, u2,v2, lr1,lg1,lb1,ls1);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32+0.5_f32, z as f32+0.5_f32, u1,v2, lr2,lg2,lb2,ls2);                              
                        
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32-0.5_f32, z as f32-0.5_f32, u2,v1, lr0,lg0,lb0,ls0);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32+0.5_f32, z as f32+0.5_f32, u1,v2, lr2,lg2,lb2,ls2);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32-0.5_f32, z as f32+0.5_f32, u1,v1, lr3,lg3,lb3,ls3);
                    }
                }
                    if let Some(Some(block)) = block {
                    let group = block.draw_group;
                    if !is_blocked(x-1,y,z, &blocks, group,&chunks){
                        //l = 0.85_f32;

                        let (u1, v1, u2, v2) = setup_uv(block.texture_faces[0] as usize, uvsize);
    
                        let lr = light(x-1,y,z, 0, &chunks) as f32 / 15.0_f32;
                        let lg = light(x-1,y,z, 1, &chunks) as f32 / 15.0_f32;
                        let lb = light(x-1,y,z, 2, &chunks) as f32 / 15.0_f32;
                        let ls = light(x-1,y,z, 3, &chunks) as f32 / 15.0_f32;
    
                        let lr0 = (light(x-1,y-1,z-1,0, &chunks) as f32 + lr*30. + light(x-1,y,z-1,0, &chunks) as f32 + light(x-1,y-1,z,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr1 = (light(x-1,y+1,z+1,0, &chunks) as f32 + lr*30. + light(x-1,y,z+1,0, &chunks) as f32 + light(x-1,y+1,z,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr2 = (light(x-1,y+1,z-1,0, &chunks) as f32 + lr*30. + light(x-1,y,z-1,0, &chunks) as f32 + light(x-1,y+1,z,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr3 = (light(x-1,y-1,z+1,0, &chunks) as f32 + lr*30. + light(x-1,y,z+1,0, &chunks) as f32 + light(x-1,y-1,z,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lg0 = (light(x-1,y-1,z-1,1, &chunks) as f32 + lg*30. + light(x-1,y,z-1,1, &chunks) as f32 + light(x-1,y-1,z,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg1 = (light(x-1,y+1,z+1,1, &chunks) as f32 + lg*30. + light(x-1,y,z+1,1, &chunks) as f32 + light(x-1,y+1,z,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg2 = (light(x-1,y+1,z-1,1, &chunks) as f32 + lg*30. + light(x-1,y,z-1,1, &chunks) as f32 + light(x-1,y+1,z,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg3 = (light(x-1,y-1,z+1,1, &chunks) as f32 + lg*30. + light(x-1,y,z+1,1, &chunks) as f32 + light(x-1,y-1,z,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lb0 = (light(x-1,y-1,z-1,2, &chunks) as f32 + lb*30. + light(x-1,y,z-1,2, &chunks) as f32 + light(x-1,y-1,z,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb1 = (light(x-1,y+1,z+1,2, &chunks) as f32 + lb*30. + light(x-1,y,z+1,2, &chunks) as f32 + light(x-1,y+1,z,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb2 = (light(x-1,y+1,z-1,2, &chunks) as f32 + lb*30. + light(x-1,y,z-1,2, &chunks) as f32 + light(x-1,y+1,z,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb3 = (light(x-1,y-1,z+1,2, &chunks) as f32 + lb*30. + light(x-1,y,z+1,2, &chunks) as f32 + light(x-1,y-1,z,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let ls0 = (light(x-1,y-1,z-1,3, &chunks) as f32 + ls*30. + light(x-1,y,z-1,3, &chunks) as f32 + light(x-1,y-1,z,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls1 = (light(x-1,y+1,z+1,3, &chunks) as f32 + ls*30. + light(x-1,y,z+1,3, &chunks) as f32 + light(x-1,y+1,z,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls2 = (light(x-1,y+1,z-1,3, &chunks) as f32 + ls*30. + light(x-1,y,z-1,3, &chunks) as f32 + light(x-1,y+1,z,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls3 = (light(x-1,y-1,z+1,3, &chunks) as f32 + ls*30. + light(x-1,y,z+1,3, &chunks) as f32 + light(x-1,y-1,z,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32-0.5_f32, z as f32-0.5_f32, u1,v1, lr0,lg0,lb0,ls0);                         
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32+0.5_f32, z as f32+0.5_f32, u2,v2, lr1,lg1,lb1,ls1);                         
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32+0.5_f32, z as f32-0.5_f32, u1,v2, lr2,lg2,lb2,ls2);                

                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32-0.5_f32, z as f32-0.5_f32, u1,v1, lr0,lg0,lb0,ls0);                         
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32-0.5_f32, z as f32+0.5_f32, u2,v1, lr3,lg3,lb3,ls3);                         
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32+0.5_f32, z as f32+0.5_f32, u2,v2, lr1,lg1,lb1,ls1);
                    }
                }
    
                    if let Some(Some(block)) = block {
                    let group = block.draw_group;
                    if !is_blocked(x,y,z+1, &blocks, group,&chunks){
                        l = 0.9_f32;

                        let (u1, v1, u2, v2) = setup_uv(block.texture_faces[5] as usize, uvsize);
    
                        let lr = light(x,y,z+1, 0, &chunks) as f32 / 15.0_f32;
                        let lg = light(x,y,z+1, 1, &chunks) as f32 / 15.0_f32;
                        let lb = light(x,y,z+1, 2, &chunks) as f32 / 15.0_f32;
                        let ls = light(x,y,z+1, 3, &chunks) as f32 / 15.0_f32;
    
                        let lr0 = l*(light(x-1,y-1,z+1,0, &chunks) as f32 + lr*30. + light(x,y-1,z+1,0, &chunks) as f32 + light(x-1,y,z+1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr1 = l*(light(x+1,y+1,z+1,0, &chunks) as f32 + lr*30. + light(x,y+1,z+1,0, &chunks) as f32 + light(x+1,y,z+1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr2 = l*(light(x-1,y+1,z+1,0, &chunks) as f32 + lr*30. + light(x,y+1,z+1,0, &chunks) as f32 + light(x-1,y,z+1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr3 = l*(light(x+1,y-1,z+1,0, &chunks) as f32 + lr*30. + light(x,y-1,z+1,0, &chunks) as f32 + light(x+1,y,z+1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lg0 = l*(light(x-1,y-1,z+1,1, &chunks) as f32 + lg*30. + light(x,y-1,z+1,1, &chunks) as f32 + light(x-1,y,z+1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg1 = l*(light(x+1,y+1,z+1,1, &chunks) as f32 + lg*30. + light(x,y+1,z+1,1, &chunks) as f32 + light(x+1,y,z+1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg2 = l*(light(x-1,y+1,z+1,1, &chunks) as f32 + lg*30. + light(x,y+1,z+1,1, &chunks) as f32 + light(x-1,y,z+1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg3 = l*(light(x+1,y-1,z+1,1, &chunks) as f32 + lg*30. + light(x,y-1,z+1,1, &chunks) as f32 + light(x+1,y,z+1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lb0 = l*(light(x-1,y-1,z+1,2, &chunks) as f32 + lb*30. + light(x,y-1,z+1,2, &chunks) as f32 + light(x-1,y,z+1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb1 = l*(light(x+1,y+1,z+1,2, &chunks) as f32 + lb*30. + light(x,y+1,z+1,2, &chunks) as f32 + light(x+1,y,z+1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb2 = l*(light(x-1,y+1,z+1,2, &chunks) as f32 + lb*30. + light(x,y+1,z+1,2, &chunks) as f32 + light(x-1,y,z+1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb3 = l*(light(x+1,y-1,z+1,2, &chunks) as f32 + lb*30. + light(x,y-1,z+1,2, &chunks) as f32 + light(x+1,y,z+1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let ls0 = l*(light(x-1,y-1,z+1,3, &chunks) as f32 + ls*30. + light(x,y-1,z+1,3, &chunks) as f32 + light(x-1,y,z+1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls1 = l*(light(x+1,y+1,z+1,3, &chunks) as f32 + ls*30. + light(x,y+1,z+1,3, &chunks) as f32 + light(x+1,y,z+1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls2 = l*(light(x-1,y+1,z+1,3, &chunks) as f32 + ls*30. + light(x,y+1,z+1,3, &chunks) as f32 + light(x-1,y,z+1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls3 = l*(light(x+1,y-1,z+1,3, &chunks) as f32 + ls*30. + light(x,y-1,z+1,3, &chunks) as f32 + light(x+1,y,z+1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32-0.5_f32, z as f32+0.5_f32, u1,v1, lr0,lg0,lb0,ls0);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32+0.5_f32, z as f32+0.5_f32, u2,v2, lr1,lg1,lb1,ls1);                         
                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32+0.5_f32, z as f32+0.5_f32, u1,v2, lr2,lg2,lb2,ls2);   

                        vertex(&mut self.buffer, x as f32-0.5_f32, y as f32-0.5_f32, z as f32+0.5_f32, u1,v1, lr0,lg0,lb0,ls0);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32-0.5_f32, z as f32+0.5_f32, u2,v1, lr3,lg3,lb3,ls3);                         
                        vertex(&mut self.buffer, x as f32+0.5_f32, y as f32+0.5_f32, z as f32+0.5_f32, u2,v2, lr1,lg1,lb1,ls1);
                    }
                }
                    if let Some(Some(block)) = block {
                    let group = block.draw_group;
                    if !is_blocked(x,y,z-1, &blocks, group,&chunks){
                        l = 0.8_f32;

                        let (u1, v1, u2, v2) = setup_uv(block.texture_faces[4] as usize, uvsize);
    
                        let lr = light(x,y,z-1, 0, &chunks) as f32 / 15.0_f32;
                        let lg = light(x,y,z-1, 1, &chunks) as f32 / 15.0_f32;
                        let lb = light(x,y,z-1, 2, &chunks) as f32 / 15.0_f32;
                        let ls = light(x,y,z-1, 3, &chunks) as f32 / 15.0_f32;
    
                        let lr0 = l*(light(x-1,y-1,z-1,0, &chunks) as f32 + lr*30_f32 + light(x,y-1,z-1,0, &chunks) as f32 + light(x-1,y,z-1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr1 = l*(light(x-1,y+1,z-1,0, &chunks) as f32 + lr*30_f32 + light(x,y+1,z-1,0, &chunks) as f32 + light(x-1,y,z-1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr2 = l*(light(x+1,y+1,z-1,0, &chunks) as f32 + lr*30_f32 + light(x,y+1,z-1,0, &chunks) as f32 + light(x+1,y,z-1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lr3 = l*(light(x+1,y-1,z-1,0, &chunks) as f32 + lr*30_f32 + light(x,y-1,z-1,0, &chunks) as f32 + light(x+1,y,z-1,0, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lg0 = l*(light(x-1,y-1,z-1,1, &chunks) as f32 + lg*30_f32 + light(x,y-1,z-1,1, &chunks) as f32 + light(x-1,y,z-1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg1 = l*(light(x-1,y+1,z-1,1, &chunks) as f32 + lg*30_f32 + light(x,y+1,z-1,1, &chunks) as f32 + light(x-1,y,z-1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg2 = l*(light(x+1,y+1,z-1,1, &chunks) as f32 + lg*30_f32 + light(x,y+1,z-1,1, &chunks) as f32 + light(x+1,y,z-1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lg3 = l*(light(x+1,y-1,z-1,1, &chunks) as f32 + lg*30_f32 + light(x,y-1,z-1,1, &chunks) as f32 + light(x+1,y,z-1,1, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let lb0 = l*(light(x-1,y-1,z-1,2, &chunks) as f32 + lb*30_f32 + light(x,y-1,z-1,2, &chunks) as f32 + light(x-1,y,z-1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb1 = l*(light(x-1,y+1,z-1,2, &chunks) as f32 + lb*30_f32 + light(x,y+1,z-1,2, &chunks) as f32 + light(x-1,y,z-1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb2 = l*(light(x+1,y+1,z-1,2, &chunks) as f32 + lb*30_f32 + light(x,y+1,z-1,2, &chunks) as f32 + light(x+1,y,z-1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let lb3 = l*(light(x+1,y-1,z-1,2, &chunks) as f32 + lb*30_f32 + light(x,y-1,z-1,2, &chunks) as f32 + light(x+1,y,z-1,2, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        let ls0 = l*(light(x-1,y-1,z-1,3, &chunks) as f32 + ls*30_f32 + light(x,y-1,z-1,3, &chunks) as f32 + light(x-1,y,z-1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls1 = l*(light(x-1,y+1,z-1,3, &chunks) as f32 + ls*30_f32 + light(x,y+1,z-1,3, &chunks) as f32 + light(x-1,y,z-1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls2 = l*(light(x+1,y+1,z-1,3, &chunks) as f32 + ls*30_f32 + light(x,y+1,z-1,3, &chunks) as f32 + light(x+1,y,z-1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
                        let ls3 = l*(light(x+1,y-1,z-1,3, &chunks) as f32 + ls*30_f32  + light(x,y-1,z-1,3, &chunks) as f32 + light(x+1,y,z-1,3, &chunks) as f32) / 5.0_f32 / 15.0_f32;
    
                        vertex(&mut self.buffer, x as f32 -0.5_f32, y as f32-0.5_f32, z as f32-0.5_f32, u2,v1, lr0,lg0,lb0,ls0);                         
                        vertex(&mut self.buffer, x as f32 -0.5_f32, y as f32+0.5_f32, z as f32-0.5_f32, u2,v2, lr1,lg1,lb1,ls1);                         
                        vertex(&mut self.buffer, x as f32 +0.5_f32, y as f32+0.5_f32, z as f32-0.5_f32, u1,v2, lr2,lg2,lb2,ls2);                              
                        
                        vertex(&mut self.buffer, x as f32 -0.5_f32, y as f32-0.5_f32, z as f32-0.5_f32, u2,v1, lr0,lg0,lb0,ls0);                         
                        vertex(&mut self.buffer, x as f32 +0.5_f32, y as f32 +0.5_f32, z as f32-0.5_f32, u1,v2, lr2,lg2,lb2,ls2);                         
                        vertex(&mut self.buffer, x as f32 +0.5_f32, y as f32 -0.5_f32, z as f32-0.5_f32, u1,v1, lr3,lg3,lb3,ls3);
                    }
                }
                }
            }
        }

        Mesh::new(self.buffer.as_ptr(), self.buffer.len() / VERTEX_SIZE, [3, 2, 4, 0].as_ptr())
    }
}
