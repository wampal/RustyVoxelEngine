use super::mesh::Mesh;

pub struct LineBatch {
    capacity: usize,
    buffer: Vec<f32>,
    mesh: Mesh,
}

const LB_VERTEX_SIZE: usize = 7;

impl LineBatch {
    pub fn new(capacity: usize) -> Self {
        let buffer = Vec::with_capacity(capacity * LB_VERTEX_SIZE * 2);
        let attrs = [3, 4, 0];
        let mesh = Mesh::new(buffer.as_ptr(), 0, attrs.as_ptr());
        Self {
            capacity,
            buffer,
            mesh,
        }
    }

    pub fn line(&mut self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32,
                r: f32, g: f32, b: f32, a: f32) {
        if self.buffer.len() >= self.capacity * LB_VERTEX_SIZE * 2 {
            return;
        }
        self.buffer.push(x1);
        self.buffer.push(y1);
        self.buffer.push(z1);
        self.buffer.push(r);
        self.buffer.push(g);
        self.buffer.push(b);
        self.buffer.push(a);

        self.buffer.push(x2);
        self.buffer.push(y2);
        self.buffer.push(z2);
        self.buffer.push(r);
        self.buffer.push(g);
        self.buffer.push(b);
        self.buffer.push(a);
    }

    pub fn boxx(&mut self, x: f32, y: f32, z: f32, w: f32, h: f32, d: f32,
               r: f32, g: f32, b: f32, a: f32) {
        let w_half = w * 0.5;
        let h_half = h * 0.5;
        let d_half = d * 0.5;

        self.line(x - w_half, y - h_half, z - d_half, x + w_half, y - h_half, z - d_half, r, g, b, a);
        self.line(x - w_half, y + h_half, z - d_half, x + w_half, y + h_half, z - d_half, r, g, b, a);
        self.line(x - w_half, y - h_half, z + d_half, x + w_half, y - h_half, z + d_half, r, g, b, a);
        self.line(x - w_half, y + h_half, z + d_half, x + w_half, y + h_half, z + d_half, r, g, b, a);

        self.line(x - w_half, y - h_half, z - d_half, x - w_half, y + h_half, z - d_half, r, g, b, a);
        self.line(x + w_half, y - h_half, z - d_half, x + w_half, y + h_half, z - d_half, r, g, b, a);
        self.line(x - w_half, y - h_half, z + d_half, x - w_half, y + h_half, z + d_half, r, g, b, a);
        self.line(x + w_half, y - h_half, z + d_half, x + w_half, y + h_half, z + d_half, r, g, b, a);

        self.line(x - w_half, y - h_half, z - d_half, x - w_half, y - h_half, z + d_half, r, g, b, a);
        self.line(x + w_half, y - h_half, z - d_half, x + w_half, y - h_half, z + d_half, r, g, b, a);
        self.line(x - w_half, y + h_half, z - d_half, x - w_half, y + h_half, z + d_half, r, g, b, a);
        self.line(x + w_half, y + h_half, z - d_half, x + w_half, y + h_half, z + d_half, r, g, b, a);
    }

    pub fn render(&mut self) {
        if self.buffer.len() == 0 {
            return;
        }
        self.mesh.reload(self.buffer.as_ptr(), self.buffer.len() / LB_VERTEX_SIZE);
        self.mesh.draw(gl::LINES);
        self.buffer.clear();
    }
}