use crate::voxels::{ chunks::Chunks, BlockRegistry, CHUNK_D, CHUNK_H, CHUNK_VOL, CHUNK_W };

use self::light_solver::LightSolver;

pub mod light_solver;
pub mod lightmap;

pub struct Lighting {
    solver_r: LightSolver,
    solver_g: LightSolver,
    solver_b: LightSolver,
    solver_s: LightSolver,
}
impl Lighting {
    pub fn new() -> Self {
        let solver_r = LightSolver::new(0);
        let solver_g = LightSolver::new(1);
        let solver_b = LightSolver::new(2);
        let solver_s = LightSolver::new(3);
        Lighting {
            solver_r,
            solver_g,
            solver_b,
            solver_s,
        }
    }

    pub fn clear(&mut self, chunks: &mut Chunks) {
        for y in 0..chunks.h as i32 {
            for z in 0..chunks.d as i32 {
                for x in 0..chunks.w as i32 {
                    if let Some(chunk) = chunks.get_mut_chunk(x, y, z) {
                        for i in 0..CHUNK_VOL {
                            chunk.lightmap.map[i] = 0;
                        }
                    }
                }
            }
        }
    }

    pub fn on_world_loaded(&mut self, blocks: &BlockRegistry, chunks: &mut Chunks) {
        let h = chunks.h;
        let d = chunks.d;
        let w = chunks.w;

        for y in 0..h as i32 * CHUNK_H {
            for z in 0..d as i32 * CHUNK_D {
                for x in 0..w as i32 * CHUNK_W {
                    let vox = chunks.get_voxel(x, y, z);
                    if let Some(vox) = vox {
                        if vox.id == 3 {
                            self.solver_r.add(x, y, z, Some(15), chunks);
                            self.solver_g.add(x, y, z, Some(15), chunks);
                            self.solver_b.add(x, y, z, Some(15), chunks);
                        }
                    }
                }
            }
        }

        for z in 0..d as i32 * CHUNK_D {
            for x in 0..w as i32 * CHUNK_W {
                for y in 0..h as i32 * CHUNK_H {
                    let vox = chunks.get_voxel(x, y, z);
                    if let Some(vox) = vox {
                        if vox.id != 0 {
                            break;
                        }
                        let voxel = chunks.get_mut_chunk_by_voxel(
                            x,
                            y,
                            z 
                        );
                        if let Some(voxel) = voxel {
                            voxel.lightmap.set_s(x % CHUNK_W, y % CHUNK_H, z % CHUNK_D, 0xf);
                        }
                    }
                }
            }
        }

        for z in 0..d as i32 * CHUNK_D {
            for x in 0..w as i32 * CHUNK_W {
                for y in (0..=h as i32* CHUNK_H - 1).rev() {
                    let vox = chunks.get_voxel(x, y, z);
                    if let Some(vox) = vox {
                        if vox.id != 0 {
                            break;
                        }

                        if
                            chunks.get_light(x - 1, y, z, 3) == 0 ||
                            chunks.get_light(x + 1, y, z, 3) == 0 ||
                            chunks.get_light(x, y - 1, z, 3) == 0 ||
                            chunks.get_light(x, y + 1, z, 3) == 0 ||
                            chunks.get_light(x, y, z - 1, 3) == 0 ||
                            chunks.get_light(x, y, z + 1, 3) == 0
                        {
                            self.solver_s.add(x, y, z, None, chunks);
                        }
                        if
                            let Some(voxel) = chunks.get_mut_chunk_by_voxel(
                                x,
                                y,
                                z
                            )
                        {
                            voxel.lightmap.set_s(x % CHUNK_W, y % CHUNK_H, z % CHUNK_D, 0xf);
                        }
                    }
                }
            }
        }

        self.solver_r.solve(&blocks, chunks);
        self.solver_g.solve(&blocks, chunks);
        self.solver_b.solve(&blocks, chunks);
        self.solver_s.solve(&blocks, chunks);
    }

    pub fn on_block_set(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        id: u8,
        blocks: &BlockRegistry,
        chunks: &mut Chunks
    ) {
        if id == 0 {
            self.solver_r.remove(x, y, z, chunks);
            self.solver_g.remove(x, y, z, chunks);
            self.solver_b.remove(x, y, z, chunks);

            self.solver_r.solve(&blocks, chunks);
            self.solver_g.solve(&blocks, chunks);
            self.solver_b.solve(&blocks, chunks);

            if chunks.get_light(x, y + 1, z, 3) == 0xf {
                for i in (0..=y).rev() {
                    let voxel = chunks.get_voxel(x, i, z);
                    if let Some(voxel) = voxel {
                        if voxel.id != 0 {
                            break;
                        }
                        self.solver_s.add(x, i, z, Some(0xf), chunks);
                    }
                }
            }
            let (x, y, z) = (x, y, z);
            self.solver_r.add(x, y + 1, z, None, chunks);
            self.solver_g.add(x, y + 1, z, None, chunks);
            self.solver_b.add(x, y + 1, z, None, chunks);
            self.solver_s.add(x, y + 1, z, None, chunks);
            self.solver_r.add(x, y - 1, z, None, chunks);
            self.solver_g.add(x, y - 1, z, None, chunks);
            self.solver_b.add(x, y - 1, z, None, chunks);
            self.solver_s.add(x, y - 1, z, None, chunks);
            self.solver_r.add(x + 1, y, z, None, chunks);
            self.solver_g.add(x + 1, y, z, None, chunks);
            self.solver_b.add(x + 1, y, z, None, chunks);
            self.solver_s.add(x + 1, y, z, None, chunks);
            self.solver_r.add(x - 1, y, z, None, chunks);
            self.solver_g.add(x - 1, y, z, None, chunks);
            self.solver_b.add(x - 1, y, z, None, chunks);
            self.solver_s.add(x - 1, y, z, None, chunks);
            self.solver_r.add(x, y, z + 1, None, chunks);
            self.solver_g.add(x, y, z + 1, None, chunks);
            self.solver_b.add(x, y, z + 1, None, chunks);
            self.solver_s.add(x, y, z + 1, None, chunks);
            self.solver_r.add(x, y, z - 1, None, chunks);
            self.solver_g.add(x, y, z - 1, None, chunks);
            self.solver_b.add(x, y, z - 1, None, chunks);
            self.solver_s.add(x, y, z - 1, None, chunks);

            self.solver_r.solve(&blocks, chunks);
            self.solver_g.solve(&blocks, chunks);
            self.solver_b.solve(&blocks, chunks);
            self.solver_s.solve(&blocks, chunks);
        } else {
            self.solver_r.remove(x, y, z, chunks);
            self.solver_g.remove(x, y, z, chunks);
            self.solver_b.remove(x, y, z, chunks);
            self.solver_s.remove(x, y, z, chunks);

            for i in (0..=y - 1).rev() {
                self.solver_s.remove(x, i, z, chunks);
                if let Some(voxel) = chunks.get_voxel(x, i - 1, z) {
                    if i == 0 || voxel.id != 0 {
                        break;
                    }
                }
            }
            self.solver_r.solve(&blocks, chunks);
            self.solver_g.solve(&blocks, chunks);
            self.solver_b.solve(&blocks, chunks);
            self.solver_s.solve(&blocks, chunks);

            let block = blocks.get(id);
            if let Some(block) = block {
                if block.emission[0] != 0 || block.emission[0] != 0 || block.emission[0] != 0 {
                    let (x, y, z) = (x, y, z);
                    self.solver_r.add(x, y, z, Some(block.emission[0] as i32), chunks);
                    self.solver_g.add(x, y, z, Some(block.emission[1] as i32), chunks);
                    self.solver_b.add(x, y, z, Some(block.emission[2] as i32), chunks);

                    self.solver_r.solve(&blocks, chunks);
                    self.solver_g.solve(&blocks, chunks);
                    self.solver_b.solve(&blocks, chunks);
                }
            }
        }
    }
}
