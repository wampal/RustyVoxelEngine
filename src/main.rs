use std::borrow::Borrow;

use files::read_binary_file;
use gl::{ DEPTH_BUFFER_BIT, DEPTH_TEST, LINES, TRIANGLES };
use glam::{ vec3, Mat4, Quat, Vec3 };
use glfw::ffi::{
    KEY_0,
    KEY_A,
    KEY_D,
    KEY_ESCAPE,
    KEY_F1,
    KEY_F2,
    KEY_S,
    KEY_TAB,
    KEY_W,
    MOUSE_BUTTON_1,
    MOUSE_BUTTON_2,
};
use graphics::{ linebatch::LineBatch, load_shader, mesh::Mesh, voxel_renderer::VoxelRenderer };
use lighting::Lighting;
use loaders::png_loading::load_texture;
use voxels::{ chunks::Chunks, Block, BlockRegistry, Chunk, CHUNK_D, CHUNK_H, CHUNK_W };
use window::{ events::Events, Window };

use crate::{ files::write_binary_file, voxels::CHUNK_VOL, window::Camera };

mod window;
mod graphics;
mod loaders;
mod files;
mod voxels;
mod lighting;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const VERTICES: [f32; 8] = [
    // x   | y
    -0.01, -0.01, 0.01, 0.01,

    -0.01, 0.01, 0.01, -0.01,
];
#[allow(non_upper_case_globals)]
const attrs: [i32; 2] = [2, 0]; // null terminator

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Window 2.0").unwrap();
    let mut events = Events::new();

    events.initialize(&mut window);

    let shader = load_shader("res/main.glslv", "res/main.glslf").expect("Failed to load shader");
    let cross_shader = load_shader("res/crosshair.glslv", "res/crosshair.glslf").expect(
        "Failed to load crosshair shader"
    );

    let lines_shader = load_shader("res/lines.glslv", "res/lines.glslf").expect(
        "Failed to load lines shader"
    );

    let texture = load_texture("res/block.png").expect("Failed to load texture");

    let mut block_registry = BlockRegistry::new();

    // AIR
    let mut block = Block::new(0, 0);
    block.draw_group = 1;
    block.light_passing = true;
    block_registry.blocks[block.id as usize] = Some(block.clone());

    // STONE
    block = Block::new(1, 2);
    block_registry.blocks[block.id as usize] = Some(block.clone());

    // GRASS
    block = Block::new(2, 4);
    block.texture_faces[2] = 2;
    block.texture_faces[3] = 1;
    block_registry.blocks[block.id as usize] = Some(block.clone());

    // LAMP
    block = Block::new(3, 3);
    block.emission[0] = 10;
    block.emission[1] = 0;
    block.emission[2] = 0;
    block_registry.blocks[block.id as usize] = Some(block.clone());

    // GLASS
    block = Block::new(4, 5);
    block.draw_group = 2;
    block.light_passing = true;
    block_registry.blocks[block.id as usize] = Some(block.clone());

    // GLASS
    block = Block::new(5, 6);
    block_registry.blocks[block.id as usize] = Some(block.clone());

    let mut chunks = Chunks::new(4, 4, 4);
    let mut meshes = Vec::with_capacity(chunks.volume);
    for _ in 0..chunks.volume {
        meshes.push(None);
    }
    let mut renderer = VoxelRenderer::new(1024 * 1024 * 8);
    let mut line_batch = LineBatch::new(4096);

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Enable(DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let crosshair = Mesh::new(VERTICES.as_ptr(), 4, attrs.as_ptr());
    // Создание камеры
    let mut camera = Camera::new(Vec3::new(96.0, 16.0, 96.0), 90.0_f32.to_radians());

    // Инициализация времени
    let mut last_time = window.glfw.get_time();
    let mut _delta: f32 = 0.0;

    // Начальные координаты камеры
    let mut cam_x = 0.0;
    let mut cam_y = 0.0;

    // Скорость движения
    let speed = 15.0;

    let mut choosen_block: i32 = 1;

    let mut lighting = Lighting::new();

    lighting.on_world_loaded(&block_registry, &mut chunks);
    while !window.should_close() {
        let current_time = window.glfw.get_time();
        _delta = (current_time - last_time) as f32;
        last_time = current_time;

        if events.jpressed(KEY_ESCAPE) {
            window.set_should_close(true);
        }
        if events.jpressed(KEY_TAB) {
            window.window.set_cursor_mode(events.toggle_cursor());
        }

        for i in 0..6 {
            if events.jpressed(KEY_0 + i) {
                choosen_block = i;
            }
        }
        if events.jpressed(KEY_F1) {
            let mut buffer = vec![0u8; chunks.volume * CHUNK_VOL];
            chunks.write(&mut buffer);
            let _result = write_binary_file("world.bin", &buffer);
            println!("world saved in {} bytes", chunks.volume * CHUNK_VOL);
        }

        if events.jpressed(KEY_F2) {
            let mut buffer = vec![0u8; chunks.volume * CHUNK_VOL];
            let _result = read_binary_file("world.bin", &mut buffer);
            chunks.read(&buffer);

            lighting.clear(&mut chunks);
            lighting.on_world_loaded(&block_registry, &mut chunks);
        }
        if events.pressed(KEY_W) {
            camera.position += camera.front * _delta * speed;
        }
        if events.pressed(KEY_S) {
            camera.position -= camera.front * _delta * speed;
        }
        if events.pressed(KEY_D) {
            camera.position += camera.right * _delta * speed;
        }
        if events.pressed(KEY_A) {
            camera.position -= camera.right * _delta * speed;
        }

        if events.cursor_locked {
            cam_y += -events.delta_y / (window.height() as f32) * 2.0;
            cam_x += -events.delta_x / (window.height() as f32) * 2.0;

            if cam_y < -89.0_f32.to_radians() {
                cam_y = -89.0_f32.to_radians();
            }
            if cam_y > 89.0_f32.to_radians() {
                cam_y = 89.0_f32.to_radians();
            }

            camera.rotation = Quat::IDENTITY;
            camera.rotate(cam_y, cam_x, 0.0);
        }

        let mut end = Vec3::default();
        let mut norm = Vec3::default();
        let mut iend = Vec3::default();
        if
            let Some(_vox) = chunks.ray_cast(
                camera.position,
                camera.front,
                10.0,
                &mut end,
                &mut norm,
                &mut iend
            )
        {
            line_batch.boxx(
                iend.x + 0.5,
                iend.y + 0.5,
                iend.z + 0.5,
                1.005,
                1.005,
                1.005,
                0.0,
                0.0,
                0.0,
                0.5
            );

            if events.jclicked(MOUSE_BUTTON_1) {
                let x = iend.x as i32;
                let y = iend.y as i32;
                let z = iend.z as i32;

                chunks.set(x, y, z, 0);

                lighting.on_block_set(x, y, z, 0, &block_registry, &mut chunks);
            }
            if events.jclicked(MOUSE_BUTTON_2) {
                let x = (iend.x + norm.x) as i32;
                let y = (iend.y + norm.y) as i32;
                let z = (iend.z + norm.z) as i32;
                chunks.set(x, y, z, choosen_block);

                lighting.on_block_set(x, y, z, choosen_block as u8, &block_registry, &mut chunks);
            }
        }

        let mut closes: Vec<Option<Chunk>> = vec![None; 27];

        for i in 0..chunks.volume {
            if let Some(chunk) = chunks.chunks.get_mut(i) {
                if !chunk.modified {
                    continue;
                }
                chunk.modified = false;
            }
            let chunk = &chunks.chunks[i];

            if let Some(mesh) = meshes[i].take() {
                // Освобождаем ресурсы меша
                drop(mesh);
            }

            // Инициализируем массив closes снова
            for elem in &mut closes {
                *elem = None;
            }

            for j in 0..chunks.volume {
                let other = &chunks.chunks[j];
                let ox = other.x - chunk.x;
                let oy = other.y - chunk.y;
                let oz = other.z - chunk.z;

                if ox.abs() > 1 || oy.abs() > 1 || oz.abs() > 1 {
                    continue;
                }

                let index = ((oy + 1) * 3 + (oz + 1)) * 3 + (ox + 1);
                closes[index as usize] = Some(other.clone());
            }

            let mesh = renderer.render(chunk, &closes, &block_registry);
            meshes[i] = Some(mesh);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }
        // Используем шейдер
        shader.use_shader();

        shader.uniform_matrix(
            "projview",
            camera.get_projection(window.width() as f32, window.height() as f32) * camera.get_view()
        );

        // Привязываем текстуру
        texture.bind();

        let mut _model = Mat4::IDENTITY;
        for i in 0..chunks.volume {
            let chunk = &chunks.chunks[i];
            let mesh = meshes[i].borrow();
            _model =
                Mat4::IDENTITY *
                Mat4::from_translation(
                    vec3(
                        (chunk.x as f32) * (CHUNK_W as f32) + 0.5,
                        (chunk.y as f32) * (CHUNK_H as f32) + 0.5,
                        (chunk.z as f32) * (CHUNK_D as f32) + 0.5
                    )
                );
            shader.uniform_matrix("model", _model);
            if let Some(mesh) = mesh {
                mesh.draw(TRIANGLES);
            }
        }

        cross_shader.use_shader();
        crosshair.draw(LINES);

        lines_shader.use_shader();
        shader.uniform_matrix(
            "projview",
            camera.get_projection(window.width() as f32, window.height() as f32) * camera.get_view()
        );
        unsafe {
            gl::LineWidth(2.0);
        }
        line_batch.render();

        window.swap_buffers();
        events.pull_events(&mut window);
    }
    window.terminate();
}
