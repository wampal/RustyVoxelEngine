use gl::types::*;
use image::codecs::png;
use image::ImageDecoder;
use std::fs::File;
use std::io::BufReader;
use std::os::raw::c_void;

use crate::graphics::Texture;

fn load_png(file: &str) -> Result<(Vec<u8>, i32, i32), image::error::ImageError> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let decoder = png::PngDecoder::new(reader)?;

    let dimensions = decoder.dimensions();
    let (width, height) = (dimensions.0 as i32, dimensions.1 as i32);

    let mut buf = vec![0; width as usize * height as usize * 4]; // 4 - channels RGBA

    decoder.read_image(&mut buf)?;

    let mut flipped_buf = vec![0; buf.len()];
    let row_size = (width as usize) * 4;
    flipped_buf
        .chunks_exact_mut(row_size)
        .rev()
        .enumerate()
        .for_each(|(i, dest_row)| {
            let src_offset = i * row_size;
            dest_row.copy_from_slice(&buf[src_offset..src_offset + row_size]);
        });

    Ok((flipped_buf, width, height))
}

fn create_texture_from_png(data: Vec<u8>, width: i32, height: i32) -> GLuint {
    let mut texture: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            width,
            height,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const c_void
        );

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST_MIPMAP_LINEAR as GLint
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 4);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    texture
}

pub fn load_texture(filename: &str) -> Result<Texture, String> {
    let (data, width, height) = match load_png(filename) {
        Ok((data, width, height)) => (data, width, height),
        Err(err) => {
            return Err(format!("Error loading PNG: {}", err));
        }
    };

    let texture_id = create_texture_from_png(data, width, height);
    Ok(Texture::new(texture_id, width, height))
}
